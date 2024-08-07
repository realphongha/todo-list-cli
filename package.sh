ARCH=$(uname -m)
OS=$(uname -s)
VERSION=v0.1.2

cargo build --release

mkdir todo-list-cli-$VERSION-$ARCH-$OS

echo 'echo "Installing todo-list-cli..."' >> todo-list-cli-$VERSION-$ARCH-$OS/install.sh
echo 'cp todo /usr/local/bin/' >> todo-list-cli-$VERSION-$ARCH-$OS/install.sh
echo 'echo "Installed!"' >> todo-list-cli-$VERSION-$ARCH-$OS/install.sh
chmod +x todo-list-cli-$VERSION-$ARCH-$OS/install.sh

echo 'echo "Uninstalling todo-list-cli..."' >> todo-list-cli-$VERSION-$ARCH-$OS/uninstall.sh
echo 'rm /usr/local/bin/todo' >> todo-list-cli-$VERSION-$ARCH-$OS/uninstall.sh
echo 'echo "Uninstalled!"' >> todo-list-cli-$VERSION-$ARCH-$OS/uninstall.sh
chmod +x todo-list-cli-$VERSION-$ARCH-$OS/uninstall.sh

cp target/release/todo todo-list-cli-$VERSION-$ARCH-$OS
cp README.md todo-list-cli-$VERSION-$ARCH-$OS

zip -r todo-list-cli-$VERSION-$ARCH-$OS.zip todo-list-cli-$VERSION-$ARCH-$OS
mv todo-list-cli-$VERSION-$ARCH-$OS.zip target

rm -rf todo-list-cli-$VERSION-$ARCH-$OS
