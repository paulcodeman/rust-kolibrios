# Компиляция программы на Rust для KolibriOS
Этот README файл описывает процесс компиляции программы на Rust для операционной системы KolibriOS. Для компиляции программы вам потребуется настроить среду разработки на Ubuntu и использовать инструментарий Rust.

## Установка Rust и Cargo
Для начала необходимо установить Rust и Cargo. Чтобы установить их, выполните следующие команды в терминале Ubuntu:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
Данная команда загрузит скрипт установки Rust и Cargo, и запустит его. Затем команда инициализирует среду окружения Rust.

## Компиляция программы на Rust
Для компиляции программы на Rust для KolibriOS используйте следующие команды в терминале в папке с проектом:

```bash
cargo build --release
cargo objcopy --release -- -O binary --binary-architecture=i386:x86 rust.kex
```
Первая команда компилирует программу на Rust в режиме "release". Вторая команда конвертирует скомпилированный исполняемый файл в формат, понятный KolibriOS.

## Запуск программы на KolibriOS
Полученный файл rust.kex можно запустить на KolibriOS. Чтобы это сделать, перенесите файл на KolibriOS и запустите его.

Это все, вы готовы компилировать программы на Rust для KolibriOS!

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=paulcodeman/rust-kolibrios&type=Date)](https://star-history.com/#paulcodeman/rust-kolibrios&Date)

