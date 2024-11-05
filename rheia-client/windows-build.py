import os
import shutil
from argparse import ArgumentParser

parser = ArgumentParser()
parser.add_argument("-v", "--version")


def generate():
    args = parser.parse_args()

    path = f'{os.path.expanduser("~")}/godot/windows-build-{args.version}'

    if os.path.exists(path):
        print(f'Path \"{path}\" already exists')
        return

    os.makedirs(path)

    print('Building dll')
    os.system('cargo b -p rheia-client --release --target x86_64-pc-windows-gnu')

    print('Building exe')
    os.system(f'godot --export-release windows_desktop {path}/Rheia.exe')

    print('Creating zip')
    shutil.make_archive(f'{os.path.expanduser("~")}/godot/windows-build-{args.version}', 'zip', path)
    print('Complited')


if __name__ == '__main__':
    generate()
