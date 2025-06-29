import os
import shutil
from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument("-v", "--version")
parser.add_argument("-p", "--path")
parser.add_argument("-z", "--zip", type=bool, default=False)


def generate():
    args = parser.parse_args()

    version = args.version
    if version is None:
        import toml  # pylint: disable=import-outside-toplevel
        with open("./rheia-client/Cargo.toml", "r") as config:
            config_data = toml.load(config)
            version = config_data["package"]["version"]

    print(f"Client version: {version}")

    path = args.path
    if path is None:
        path = f'{os.path.expanduser("~")}/Dropbox/Rheia/windows-build-{version}'

    if os.path.exists(path):
        print(f'Path \"{path}\" already exists')
        return

    os.makedirs(path, exist_ok=True)

    print('Building dll')
    res = os.system('cd ~/godot/rheia/rheia-godot/; cargo b -p rheia-client --release --target x86_64-pc-windows-gnu')
    if res != 0:
        print(f'Godot build failed: {res}')
        return

    print('Building exe')
    os.system(f'cd ~/godot/rheia/rheia-godot/; godot --export-release windows_desktop {path}/Rheia.exe')

    if args.zip:
        print('Creating zip')
        shutil.make_archive(path, 'zip', path)

    print('Complited')


if __name__ == '__main__':
    generate()
