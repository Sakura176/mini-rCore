import os

base_address = 0x80400000
step = 0x20000
linker = "src/linker.ld"

app_id = 0
apps = os.listdir("src/bin")
apps.sort()
for app in apps:
    app = app[: app.find(".")]
    lines = []
    lines_before = []
    with open(linker, "r") as f:
        for line in f.readlines():
            lines_before.append(line)
            # 替换应用加载地址
            line = line.replace(hex(base_address), hex(
                base_address + step * app_id))
            lines.append(line)
    with open(linker, "w+") as f:
        f.writelines(lines)
    # 编译应用
    os.system("cargo build --bin %s --release" % app)
    print(
        "[build.py] application %s start with address %s"
        % (app, hex(base_address + step * app_id))
    )
    # 还原成默认地址
    with open(linker, "w+") as f:
        f.writelines(lines_before)
    app_id = app_id + 1
