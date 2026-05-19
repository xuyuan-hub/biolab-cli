"""
Biolab API client — auto-captures token via browser OAuth, persists to ~/.biolab_token.
Usage:
    from biolab_api import BiolabAPI
    api = BiolabAPI()
    user = api.get_me()
    order = api.create_primer_order({...})
"""

import json
import os
import sys
import urllib.parse
import urllib.request
import urllib.error
from pathlib import Path

# Windows stdout encoding fix
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

BASE_URL = "http://8.136.56.203/api/v1"
TOKEN_VAR = "BIOLAB_TOKEN"
TOKEN_FILE = Path.home() / ".biolab_token"


def _load_token() -> str | None:
    """Try env var first, then persist file."""
    token = os.environ.get(TOKEN_VAR)
    if token:
        return token
    if TOKEN_FILE.exists():
        return TOKEN_FILE.read_text().strip()
    return None


def _save_token(token: str) -> None:
    TOKEN_FILE.write_text(token)


def _auth_flow() -> str | None:
    """Feishu OAuth via browser: starts local HTTP server to receive token callback directly."""
    import time
    import socket
    from http.server import HTTPServer, BaseHTTPRequestHandler

    # Find a free port
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("", 0))
        port = s.getsockname()[1]

    callback_url = f"http://localhost:{port}/callback"
    auth_url = f"{BASE_URL}/feishu/authorize?redirect={urllib.parse.quote(callback_url, safe='')}"

    received_token = []

    class CallbackHandler(BaseHTTPRequestHandler):
        def do_GET(self):
            parsed = urllib.parse.urlparse(self.path)
            params = urllib.parse.parse_qs(parsed.query)
            token = params.get("token", [None])[0]
            if token:
                received_token.append(token)
                self.send_response(200)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.end_headers()
                self.wfile.write(
                    b"<html><body style='font-family:sans-serif;text-align:center;padding-top:3em'>"
                    b"<h2>Login Successful</h2><p>Token has been saved. You may close this window.</p>"
                    b"</body></html>"
                )
            else:
                self.send_response(204)
                self.end_headers()

        def log_message(self, format, *args):
            pass

    HTTPServer.allow_reuse_address = True
    server = HTTPServer(("127.0.0.1", port), CallbackHandler)
    server.timeout = 2

    print("请在浏览器中打开以下链接进行飞书认证：")
    print(f"\n  {auth_url}\n")
    print("等待认证完成...")

    deadline = time.time() + 120
    while time.time() < deadline and not received_token:
        try:
            server.handle_request()
        except Exception:
            pass

    server.server_close()

    if received_token:
        TOKEN_FILE.write_text(received_token[0])
        print("认证成功！Token 已保存。")
        return received_token[0]

    print("认证超时，请重试。")
    return None


class BiolabAPI:
    def __init__(self):
        token = _load_token()
        if not token:
            token = _auth_flow()
        if not token:
            sys.exit(
                f"ERROR: 无法获取 token。请手动设置:\n"
                f"  PowerShell: $env:{TOKEN_VAR}='<token>'\n"
                f"  Bash:       export {TOKEN_VAR}='<token>'"
            )
        _save_token(token)
        self.token = token
        self.base_url = BASE_URL

    # ---- internals ----

    def _req(self, method, path, data=None):
        url = f"{self.base_url}{path}"
        headers = {
            "Authorization": f"Bearer {self.token}",
            "Content-Type": "application/json",
        }
        body = json.dumps(data).encode("utf-8") if data is not None else None
        req = urllib.request.Request(url, data=body, headers=headers, method=method)
        try:
            with urllib.request.urlopen(req) as resp:
                return json.loads(resp.read())
        except urllib.error.HTTPError as e:
            detail = e.read().decode()
            sys.exit(f"HTTP {e.code} {path}: {detail}")

    # ---- users ----

    def get_me(self):
        return self._req("GET", "/users/me")

    # ---- orders ----

    def create_primer_order(self, order: dict):
        return self._req("POST", "/orders/primer", order)

    def get_default_template(self, order_type: str | None = None):
        """Get user's default order info template."""
        qs = f"?order_type={order_type}" if order_type else ""
        return self._req("GET", f"/order-info-templates/default{qs}")

    def create_sequencing_order(self, order: dict):
        order["type"] = "sequencing"
        return self._req("POST", "/orders/sequencing", order)

    def list_orders(self, skip: int = 0, limit: int = 100):
        return self._req("GET", f"/orders/?skip={skip}&limit={limit}")

    def get_order(self, order_id: str):
        return self._req("GET", f"/orders/{order_id}")

    def download_order(self, order_id: str, output_path: str):
        url = f"{self.base_url}/orders/{order_id}/download"
        req = urllib.request.Request(url, headers={
            "Authorization": f"Bearer {self.token}",
        })
        try:
            with urllib.request.urlopen(req) as resp:
                with open(output_path, "wb") as f:
                    f.write(resp.read())
            return output_path
        except urllib.error.HTTPError as e:
            detail = e.read().decode()
            sys.exit(f"HTTP {e.code} /orders/{order_id}/download: {detail}")

    def resend_order(self, order_id: str):
        return self._req("POST", f"/orders/{order_id}/send")

    def update_order(self, order_id: str, data: dict):
        return self._req("PATCH", f"/orders/{order_id}", data)

    def download_primer_template(self, output_path: str):
        return self._download_file("/orders/primer/template", output_path)

    def download_sequencing_template(self, output_path: str):
        return self._download_file("/orders/sequencing/template", output_path)

    def upload_primer_excel(self, file_path: str):
        return self._upload_file("/orders/primer/upload-excel", file_path)

    def upload_sequencing_excel(self, file_path: str):
        return self._upload_file("/orders/sequencing/upload-excel", file_path)

    def _download_file(self, path: str, output_path: str):
        url = f"{self.base_url}{path}"
        req = urllib.request.Request(url, headers={
            "Authorization": f"Bearer {self.token}",
        })
        with urllib.request.urlopen(req) as resp:
            with open(output_path, "wb") as f:
                f.write(resp.read())
        return output_path

    def _upload_file(self, path: str, file_path: str):
        import pathlib
        fname = pathlib.Path(file_path).name
        boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW"
        with open(file_path, "rb") as f:
            content = f.read()
        body = (
            f"--{boundary}\r\n"
            f'Content-Disposition: form-data; name="file"; filename="{fname}"\r\n'
            f"Content-Type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet\r\n\r\n"
        ).encode() + content + f"\r\n--{boundary}--\r\n".encode()
        url = f"{self.base_url}{path}"
        req = urllib.request.Request(url, data=body, headers={
            "Authorization": f"Bearer {self.token}",
            "Content-Type": f"multipart/form-data; boundary={boundary}",
        }, method="POST")
        with urllib.request.urlopen(req) as resp:
            return json.loads(resp.read())

    # ---- templates ----

    def list_templates(self):
        return self._req("GET", "/order-info-templates/")

    def get_template(self, template_id: str):
        return self._req("GET", f"/order-info-templates/{template_id}")

    def create_template(self, data: dict):
        return self._req("POST", "/order-info-templates/", data)

    def update_template(self, template_id: str, data: dict):
        return self._req("PUT", f"/order-info-templates/{template_id}", data)

    def delete_template(self, template_id: str):
        return self._req("DELETE", f"/order-info-templates/{template_id}")

    def set_default_template(self, template_id: str):
        return self._req("POST", f"/order-info-templates/{template_id}/set-default")

    # ---- users ----

    def update_me(self, data: dict):
        return self._req("PATCH", "/users/me", data)

    def change_password(self, current_password: str, new_password: str):
        return self._req("PATCH", "/users/me/password", {
            "current_password": current_password,
            "new_password": new_password,
        })

    # ---- inventory ----

    def list_stocks(self, primer_name: str = None, location_id: str = None, low_stock: bool = None):
        params = []
        if primer_name:
            params.append(f"primer_name={urllib.parse.quote(primer_name)}")
        if location_id:
            params.append(f"location_id={location_id}")
        if low_stock:
            params.append("low_stock=true")
        qs = f"?{'&'.join(params)}" if params else ""
        return self._req("GET", f"/inventory/stocks{qs}")

    def get_stock_stats(self):
        return self._req("GET", "/inventory/stats")

    def get_stock(self, stock_id: str):
        return self._req("GET", f"/inventory/stocks/{stock_id}")

    def get_transactions(self, stock_id: str):
        return self._req("GET", f"/inventory/stocks/{stock_id}/transactions")

    def checkout(self, stock_id: str, quantity: float, purpose: str = "", experiment_ref: str = ""):
        return self._req("POST", f"/inventory/stocks/{stock_id}/checkout", {
            "quantity": quantity,
            "purpose": purpose,
            "experiment_ref": experiment_ref,
        })

    def checkin(self, stock_id: str, quantity: float, purpose: str = ""):
        return self._req("POST", f"/inventory/stocks/{stock_id}/checkin", {
            "quantity": quantity,
            "purpose": purpose,
        })

    def list_locations(self):
        return self._req("GET", "/inventory/locations")

    def create_location(self, name: str, parent_id: str = None):
        data = {"name": name}
        if parent_id:
            data["parent_id"] = parent_id
        return self._req("POST", "/inventory/locations", data)

    # ---- lab ----

    def get_lab(self):
        return self._req("GET", "/lab")

    def create_lab(self, name: str):
        return self._req("POST", "/lab/create", {"name": name})

    def update_lab(self, data: dict):
        return self._req("PATCH", "/lab", data)

    def list_lab_members(self):
        return self._req("GET", "/lab/members")

    def update_member_role(self, user_id: str, role: str):
        return self._req("PATCH", f"/lab/members/{user_id}", {"role": role})

    def remove_member(self, user_id: str):
        return self._req("DELETE", f"/lab/members/{user_id}")

    def invite_member(self, email: str, role: str = "member"):
        return self._req("POST", "/lab/invite", {"email": email, "role": role})

    def list_invitations(self):
        return self._req("GET", "/lab/invitations")

    def accept_invitation(self, invitation_id: str):
        return self._req("POST", f"/lab/invitations/{invitation_id}/accept")

    def decline_invitation(self, invitation_id: str):
        return self._req("POST", f"/lab/invitations/{invitation_id}/decline")

    def apply_to_join_lab(self, lab_id: str, role: str = "member"):
        return self._req("POST", f"/lab/join/{lab_id}", {"role": role})

    def list_applications(self):
        return self._req("GET", "/lab/applications")

    def approve_application(self, application_id: str):
        return self._req("POST", f"/lab/applications/{application_id}/approve")

    def reject_application(self, application_id: str):
        return self._req("POST", f"/lab/applications/{application_id}/reject")

    def list_approval_rules(self):
        return self._req("GET", "/lab/approval-rules")

    def add_approval_rule(self, data: dict):
        return self._req("POST", "/lab/approval-rules", data)

    def remove_approval_rule(self, rule_id: str):
        return self._req("DELETE", f"/lab/approval-rules/{rule_id}")


# ---- helpers ----

def format_order(order: dict) -> str:
    """Format an order response for terminal display."""
    order_type = order.get("type", "")
    is_sequencing = order_type == "sequencing"

    items = order.pop("items", [])
    lines = []
    lines.append(f"订单 ID  : {order['id']}")
    lines.append(f"类型     : {'测序' if is_sequencing else '引物合成'}")
    lines.append(f"状态     : {order['status']}")
    lines.append(f"供应商   : {order['supplier_name']}")
    lines.append(f"联系人   : {order['customer_name']}  {order['customer_phone']}")
    lines.append(f"总价     : {order.get('total_price') or 'N/A'} 元")
    lines.append(f"创建时间 : {order['created_at']}")

    if is_sequencing:
        if items:
            lines.append(f"测序样品 ({len(items)} 条):")
            for item in items:
                lines.append(
                    f"  {item['name']:16s} "
                    f"type={item.get('type', '-')}  "
                    f"vector={item.get('seq_vector', '-')}  "
                    f"测通={'是' if item.get('universal') else '否'}"
                )
        primer_items = order.get("primer_items") or []
        if primer_items:
            lines.append(f"引物合成 ({len(primer_items)} 条):")
            for p in primer_items:
                lines.append(
                    f"  {p['primer_name']:12s} {p['sequence']:32s} "
                    f"OD={p.get('scale_od', '-')}  "
                    f"{p.get('purification_method', '')}"
                )
    else:
        if items:
            lines.append(f"引物 ({len(items)} 条):")
            for item in items:
                mod = item.get("five_modification", "") or ""
                mod_str = f"  [{mod}]" if mod else ""
                lines.append(
                    f"  {item['primer_name']:12s} {item['sequence']:32s} "
                    f"{item.get('base_count', 0)}bp  {item.get('purification_method', '')}  "
                    f"{item.get('nmoles', '')}nmol{mod_str}"
                )
    return "\n".join(lines)


def format_order_brief(order: dict) -> str:
    """Single-line order summary for lists."""
    return (
        f"{order['id']}  {order['status']:8s}  {order['supplier_name']:8s}  "
        f"{str(order.get('total_price') or 'N/A'):>6s}  "
        f"{order['created_at'][:19]}  "
        f"{order.get('customer_name', '')}"
    )


# ---- CLI entry ----

USAGE = """Usage: python biolab_api.py <command> [args]

账户:
  me                              我的信息
  update-me <json>                更新个人信息（如 {"phone_number":"138xx"}）

订单:
  orders                          订单列表
  order <id>                      订单详情
  create-sequencing <json_file>   创建测序订单
  update-order <id> <json>        更新订单（如 '{"status":"received"}'）
  resend-order <id>               重发邮件（pending 状态订单）
  download-order <id> [path]      下载订单 Excel

模板:
  templates                       模板列表
  template-default <type>         默认模板（primer_synthesis / sequencing）
  create-template <json_file>     创建模板
  set-default-template <id>       设为默认模板
  download-primer-template [path] 下载引物 Excel 模板
  download-sequencing-template [path] 下载测序 Excel 模板

库存:
  stocks                          库存列表
  stock <id>                      库存详情
  stock-stats                     库存统计
  locations                       存储位置列表

课题组:
  lab                             我的课题组信息
  create-lab <name>               创建课题组
  lab-members                     成员列表
  invite-member <email> [role]    邀请成员
  invitations                     查看邀请
  accept-invite <id>              接受邀请
  decline-invite <id>             拒绝邀请
  apply-join <lab_id> [role]      申请加入课题组
  applications                    查看入组申请（PI）
  approve-app <id>                批准申请（PI）
  reject-app <id>                 拒绝申请（PI）
  approval-rules                  审批规则列表
  add-rule <json>                 添加审批规则
  remove-rule <id>                删除审批规则"""

if __name__ == "__main__":
    api = BiolabAPI()
    if len(sys.argv) < 2:
        print(USAGE)
        sys.exit(1)

    cmd = sys.argv[1]
    try:
        if cmd == "me":
            print(json.dumps(api.get_me(), ensure_ascii=False, indent=2))

        elif cmd == "update-me" and len(sys.argv) > 2:
            print(api.update_me(json.loads(sys.argv[2])))

        elif cmd == "orders":
            result = api.list_orders()
            orders = result if isinstance(result, list) else result.get("data", [])
            if not orders:
                print("暂无订单")
            else:
                for o in orders:
                    print(format_order_brief(o))

        elif cmd == "order" and len(sys.argv) > 2:
            print(format_order(api.get_order(sys.argv[2])))

        elif cmd == "create-sequencing" and len(sys.argv) > 2:
            with open(sys.argv[2], "r", encoding="utf-8") as f:
                data = json.load(f)
            print(format_order(api.create_sequencing_order(data)))

        elif cmd == "update-order" and len(sys.argv) > 3:
            print(api.update_order(sys.argv[2], json.loads(sys.argv[3])))

        elif cmd == "resend-order" and len(sys.argv) > 2:
            print(api.resend_order(sys.argv[2]))

        elif cmd == "download-order" and len(sys.argv) > 2:
            path = sys.argv[3] if len(sys.argv) > 3 else "order.xlsx"
            api.download_order(sys.argv[2], path)
            print(f"已下载到 {path}")

        elif cmd == "templates":
            result = api.list_templates()
            templates = result.get("data", []) if isinstance(result, dict) else result
            if not templates:
                print("暂无信息模板")
            else:
                for t in templates:
                    d = " [默认]" if t.get("is_default") else ""
                    ot = t.get("order_type", "") or "通用"
                    print(f"{t['id']}  {t['name']:16s}  {ot}{d}")

        elif cmd == "template-default" and len(sys.argv) > 2:
            tpl = api.get_default_template(sys.argv[2])
            print(json.dumps(tpl, ensure_ascii=False, indent=2))

        elif cmd == "create-template" and len(sys.argv) > 2:
            with open(sys.argv[2], "r", encoding="utf-8") as f:
                data = json.load(f)
            tpl = api.create_template(data)
            print(f"模板已创建: {tpl.get('id', tpl)}")

        elif cmd == "set-default-template" and len(sys.argv) > 2:
            print(api.set_default_template(sys.argv[2]))

        elif cmd == "download-primer-template":
            path = sys.argv[2] if len(sys.argv) > 2 else "primer_template.xlsx"
            api.download_primer_template(path)
            print(f"已下载到 {path}")

        elif cmd == "download-sequencing-template":
            path = sys.argv[2] if len(sys.argv) > 2 else "sequencing_template.xlsx"
            api.download_sequencing_template(path)
            print(f"已下载到 {path}")

        elif cmd == "stocks":
            if len(sys.argv) > 2:
                stock = api.get_stock(sys.argv[2])
                print(json.dumps(stock, ensure_ascii=False, indent=2))
            else:
                result = api.list_stocks()
                stocks = result.get("data", []) if isinstance(result, dict) else result
                if not stocks:
                    print("暂无库存")
                else:
                    for s in stocks:
                        print(f"{s['id']}  {s.get('primer_name',''):20s}  剩余:{s.get('remaining_quantity',0)}  位置:{s.get('location_path','')}")

        elif cmd == "stock-stats":
            print(json.dumps(api.get_stock_stats(), ensure_ascii=False, indent=2))

        elif cmd == "locations":
            locations = api.list_locations()
            if isinstance(locations, list):
                for loc in locations:
                    print(f"{loc.get('id','')}  {loc.get('name','')}  {loc.get('path','')}")
            else:
                print(json.dumps(locations, ensure_ascii=False, indent=2))

        # ---- lab ----

        elif cmd == "lab":
            print(json.dumps(api.get_lab(), ensure_ascii=False, indent=2))

        elif cmd == "create-lab" and len(sys.argv) > 2:
            print(json.dumps(api.create_lab(sys.argv[2]), ensure_ascii=False, indent=2))

        elif cmd == "lab-members":
            members = api.list_lab_members()
            for m in members:
                print(f"{m['id']}  {m.get('full_name',''):12s}  {m['email']:24s}  {m.get('role','')}")

        elif cmd == "invite-member" and len(sys.argv) > 2:
            role = sys.argv[3] if len(sys.argv) > 3 else "member"
            print(api.invite_member(sys.argv[2], role))

        elif cmd == "invitations":
            invs = api.list_invitations()
            for i in invs:
                print(f"{i['id']}  {i.get('lab_name',''):16s}  {i['invitee_email']:24s}  {i['status']}")

        elif cmd == "accept-invite" and len(sys.argv) > 2:
            print(api.accept_invitation(sys.argv[2]))

        elif cmd == "decline-invite" and len(sys.argv) > 2:
            print(api.decline_invitation(sys.argv[2]))

        elif cmd == "apply-join" and len(sys.argv) > 2:
            role = sys.argv[3] if len(sys.argv) > 3 else "member"
            print(api.apply_to_join_lab(sys.argv[2], role))

        elif cmd == "applications":
            apps = api.list_applications()
            for a in apps:
                print(f"{a['id']}  {a.get('invitee_email',''):24s}  {a.get('role','')}  {a['status']}")

        elif cmd == "approve-app" and len(sys.argv) > 2:
            print(api.approve_application(sys.argv[2]))

        elif cmd == "reject-app" and len(sys.argv) > 2:
            print(api.reject_application(sys.argv[2]))

        elif cmd == "approval-rules":
            rules = api.list_approval_rules()
            for r in rules:
                ot = r.get('order_type') or "全部"
                price = f"≤¥{r['max_price']}" if r.get('max_price') else "不限金额"
                print(f"{r['id']}  #{r.get('sort_order',0)}  {ot}  {price}  → {r['approver_role']}")

        elif cmd == "add-rule" and len(sys.argv) > 2:
            data = json.loads(sys.argv[2])
            print(api.add_approval_rule(data))

        elif cmd == "remove-rule" and len(sys.argv) > 2:
            print(api.remove_approval_rule(sys.argv[2]))

        else:
            print(f"Unknown command: {cmd}")
            print(USAGE)
            sys.exit(1)
    except SystemExit:
        raise
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)
