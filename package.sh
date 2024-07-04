ARCH=$(uname -m)
OS=$(uname -s)

cargo build --release

echo 'echo "Installing todo-list-cli..."' >> target/release/install.sh
echo 'cp todo /usr/local/bin/' >> target/release/install.sh
echo 'echo "Installed!"' >> target/release/install.sh
chmod +x target/release/install.sh

echo 'echo "Uninstalling todo-list-cli..."' >> target/release/uninstall.sh
echo 'rm /usr/local/bin/todo' >> target/release/uninstall.sh
echo 'echo "Uninstalled!"' >> target/release/uninstall.sh
chmod +x target/release/uninstall.sh

zip -j target/todo-list-cli-$ARCH-$OS.zip target/release/todo \
    target/release/install.sh target/release/uninstall.sh \
    README.md

rm target/release/install.sh
rm target/release/uninstall.sh
