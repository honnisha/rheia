import os
import shutil
from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument("-v", "--version", required=True)
parser.add_argument("-p", "--path")
parser.add_argument("-z", "--zip", type=bool, default=False)


def generate():
    args = parser.parse_args()

    path = args.path
    if path is None:
        path = f'{os.path.expanduser("~")}/Dropbox/Rheia/windows-build-{args.version}'

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
