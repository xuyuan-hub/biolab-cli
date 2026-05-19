"""
Interactive primer ordering CLI.
Reads BIOLAB_TOKEN from environment, never from CLI.

Usage:
    # Interactive mode (default)
    uv run python order_cli.py

    # Submit from JSON file
    uv run python order_cli.py --file order.json

    # Submit from JSON string
    uv run python order_cli.py --json '{"type":"primer_synthesis",...}'
"""

import json
import os
import sys
from pathlib import Path

# Windows stdout encoding fix
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))
from biolab_api import BiolabAPI, format_order

# ---- defaults (fallback, overridden by user template) ----

DEFAULTS = {
    "company_name": "",
    "invoice_title": "",
    "principal_investigator": "",
    "payment_method": "",
    "recipient_address": "",
    "weekend_delivery": True,
    "partial_delivery": True,
    "confidential": True,
}

SUPPLIERS = {"sangon": "生工", "biosune": "铂尚", "tsingke": "擎科"}


# ---- interactive ----

def _ask(prompt: str, default: str = None) -> str:
    if default:
        prompt = f"{prompt} [{default}]: "
    else:
        prompt = f"{prompt}: "
    value = input(prompt).strip()
    return value if value else (default or "")


def _ask_bool(prompt: str, default: bool = True) -> bool:
    d = "Y/n" if default else "y/N"
    val = _ask(f"{prompt} ({d})").lower()
    if not val:
        return default
    return val in ("y", "yes", "true", "1")


def _ask_supplier() -> str:
    print("\n供应商:")
    print("  1. 生工 (sangon)")
    print("  2. 铂尚 (biosune)")
    print("  3. 擎科 (tsingke)")
    choice = _ask("选择", "1")
    return {"1": "sangon", "2": "biosune", "3": "tsingke"}.get(choice, "sangon")


def _ask_primer() -> dict:
    """Ask for a single primer's info."""
    print()
    name = _ask("  引物名称")
    if not name:
        return None
    seq = _ask("  序列 (5'→3')")
    if not seq:
        return None

    item = {"primer_name": name, "sequence": seq}

    bc = _ask("  碱基数", str(len(seq)))
    item["base_count"] = int(bc) if bc else len(seq)

    pm = _ask("  纯化方式 (HAP/PAGE/HPLC/ULTRAPAGE)", "HPLC")
    if pm:
        item["purification_method"] = pm

    nm = _ask("  nmoles")
    if nm:
        item["nmoles"] = float(nm)

    od = _ask("  OD 值 (nmoles有值时跳过)")
    if od:
        item["scale_od"] = float(od)

    tc = _ask("  分装管数", "1")
    item["tube_count"] = int(tc) if tc else 1

    five = _ask("  5' 修饰 (如 5' NH2 C6，无则回车)")
    if five:
        item["five_modification"] = five

    three = _ask("  3' 修饰 (无则回车)")
    if three:
        item["three_modification"] = three

    return item


def _collect_primers() -> list:
    """Collect primers one by one until blank name."""
    items = []
    print("\n--- 引物列表（空白名称结束输入）---")
    while True:
        item = _ask_primer()
        if item is None:
            break
        items.append(item)
        print(f"  ✓ {item['primer_name']} ({item['base_count']}bp) 已添加")
    return items


def interactive_order():
    """Full interactive ordering flow."""
    api = BiolabAPI()

    # 1. Fetch user profile
    print("获取用户信息...")
    try:
        me = api.get_me()
    except SystemExit:
        raise
    except Exception as e:
        sys.exit(f"获取用户信息失败: {e}")

    print(f"当前用户: {me['full_name']}  {me.get('phone_number', '')}  {me['email']}")
    print()

    # 2. Supplier
    supplier = _ask_supplier()
    print(f"供应商: {SUPPLIERS[supplier]}\n")

    # 3. Check if user wants to change contact info
    customer_name = me["full_name"]
    customer_phone = me.get("phone_number", "")
    customer_email = me.get("email", "")

    print("--- 联系人（来自账户，回车确认）---")
    new_name = _ask("  姓名", customer_name)
    new_phone = _ask("  手机", customer_phone)
    new_email = _ask("  邮箱", customer_email)

    # 4. Fetch default template
    print("\n获取默认信息模板...")
    try:
        tpl = api.get_default_template("primer_synthesis")
    except Exception:
        tpl = {}
    if tpl:
        print(f"模板: {tpl.get('name', '')}")
        DEFAULTS["company_name"] = tpl.get("company_name") or ""
        DEFAULTS["invoice_title"] = tpl.get("invoice_title") or ""
        DEFAULTS["principal_investigator"] = tpl.get("principal_investigator") or ""
        DEFAULTS["payment_method"] = tpl.get("payment_method") or ""
        DEFAULTS["recipient_address"] = tpl.get("recipient_address") or ""

    # 5. Confirm defaults
    print("\n--- 订单默认值（回车确认，输入新值修改）---")
    order = {**DEFAULTS}
    order["customer_name"] = new_name
    order["customer_phone"] = new_phone
    order["customer_email"] = new_email

    for key in DEFAULTS:
        default = order[key]
        if isinstance(default, bool):
            order[key] = _ask_bool(f"  {key}", default)
        else:
            order[key] = _ask(f"  {key}", default)

    # 5. Collect primers
    items = _collect_primers()
    if not items:
        sys.exit("未输入任何引物，已取消。")

    # 6. Build order payload
    payload = {"type": "primer_synthesis", "supplier_name": supplier, "items": items}
    payload.update(order)

    # 7. Confirm
    print("\n" + "=" * 50)
    print(f"联系人 : {payload['customer_name']}  {payload['customer_phone']}")
    print(f"供应商 : {SUPPLIERS[payload['supplier_name']]}")
    print(f"引物数 : {len(items)} 条, 共 {sum(i.get('base_count', 0) for i in items)} 碱基")
    print(f"地址   : {payload['recipient_address']}")
    total_nmoles = sum(i.get("nmoles", 0) or 0 for i in items)
    if total_nmoles:
        print(f"总 nmol: {total_nmoles}")
    print("=" * 50)

    if not _ask_bool("\n确认提交？", True):
        sys.exit("已取消。")

    # 8. Submit
    print("\n提交中...")
    try:
        result = api.create_primer_order(payload)
        print("\n✓ 下单成功！\n")
        print(format_order(result))
    except SystemExit:
        raise
    except Exception as e:
        sys.exit(f"下单失败: {e}")


# ---- batch submit ----

def submit_order(order_data: dict):
    """Submit order from pre-built JSON. Returns result dict."""
    api = BiolabAPI()
    return api.create_primer_order(order_data)


# ---- CLI ----

def main():
    args = sys.argv[1:]

    # --file mode
    if "--file" in args:
        idx = args.index("--file")
        path = args[idx + 1] if idx + 1 < len(args) else None
        if not path:
            sys.exit("Usage: order_cli.py --file <path>")
        with open(path, "r", encoding="utf-8") as f:
            data = json.load(f)
        try:
            result = submit_order(data)
            print(format_order(result))
        except SystemExit:
            raise
        except Exception as e:
            sys.exit(f"下单失败: {e}")
        return

    # --json mode
    if "--json" in args:
        idx = args.index("--json")
        raw = args[idx + 1] if idx + 1 < len(args) else None
        if not raw:
            sys.exit("Usage: order_cli.py --json '<json_string>'")
        data = json.loads(raw)
        try:
            result = submit_order(data)
            print(format_order(result))
        except SystemExit:
            raise
        except Exception as e:
            sys.exit(f"下单失败: {e}")
        return

    # default: interactive
    try:
        interactive_order()
    except (EOFError, KeyboardInterrupt):
        print("\n\n已取消。")
        sys.exit(1)


if __name__ == "__main__":
    main()
