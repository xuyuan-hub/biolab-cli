"""
Biolab login via Feishu OAuth.
Token is saved to ~/.biolab_token (valid 8 days).

Usage:
    uv run python login.py              # interactive (starts local server, waits for callback)
    uv run python login.py --status     # check if logged in
    uv run python login.py --logout     # remove stored token
"""

import sys
import time
import json
import urllib.request
import urllib.error
import urllib.parse
from pathlib import Path

# Force UTF-8 on Windows to avoid garbled Chinese output
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")

TOKEN_FILE = Path.home() / ".biolab_token"
BASE_URL = "http://8.136.56.203/api/v1"


def check_status():
    if not TOKEN_FILE.exists():
        print("未登录（无 token 文件）")
        return False
    token = TOKEN_FILE.read_text().strip()
    req = urllib.request.Request(
        f"{BASE_URL}/users/me",
        headers={"Authorization": f"Bearer {token}"},
    )
    try:
        with urllib.request.urlopen(req) as resp:
            user = json.loads(resp.read())
        print(f"已登录: {user['full_name']} ({user['email']})")
        return True
    except urllib.error.HTTPError as e:
        print(f"Token 无效: HTTP {e.code}")
        return False


def login():
    """Start local HTTP server, print auth URL, wait for OAuth callback."""
    import socket
    from http.server import HTTPServer, BaseHTTPRequestHandler

    old_token = TOKEN_FILE.read_text().strip() if TOKEN_FILE.exists() else None
    if old_token:
        print("已有 token，尝试验证...")
        if check_status():
            print("当前 token 有效，无需重新登录。")
            print("如需重新登录，请先执行 --logout")
            return True
        print("Token 已过期，开始重新认证...\n")

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

    print("\n" + "=" * 55)
    print("  请在浏览器中打开以下链接完成飞书认证：")
    print(f"\n    {auth_url}\n")
    print("  等待认证完成（最多 2 分钟），每 5 秒打印 .")
    print("=" * 55 + "\n")

    deadline = time.time() + 120
    last_dot = time.time()
    while time.time() < deadline and not received_token:
        try:
            server.handle_request()
        except Exception:
            pass
        if time.time() - last_dot >= 5:
            print(".", end="", flush=True)
            last_dot = time.time()

    print()
    server.server_close()

    if received_token:
        TOKEN_FILE.write_text(received_token[0])
        print("认证成功！Token 已保存到 ~/.biolab_token")
        return check_status()

    print("\n认证超时（超过 2 分钟未收到回调）。")
    print("请重新运行并在浏览器中打开授权链接。")
    return False


def logout():
    if TOKEN_FILE.exists():
        TOKEN_FILE.unlink()
        print("已登出，Token 已删除。")
    else:
        print("未登录。")


def main():
    args = sys.argv[1:]
    if "--status" in args:
        sys.exit(0 if check_status() else 1)
    elif "--logout" in args:
        logout()
    else:
        success = login()
        sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
