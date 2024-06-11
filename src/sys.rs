#[link(name = "syscalls")]
extern "C" {
    // Системный вызов для завершения программы
    #[link_name = "_exit"]
    pub fn exit() -> !;

    // Системный вызов для создания окна
    #[link_name = "_define_window"]
    pub fn define_window(ebx: u32, ecx: u32, edx: u32, esi: u32, edi: u32);

    // Системный вызов для установки пикселя
    #[link_name = "_put_pixel"]
    pub fn put_pixel(ebx: u32, ecx: u32, edx: u32);

    // Системный вызов для получения нажатой клавиши
    #[link_name = "_pressed_key"]
    pub fn pressed_key() -> u32;

    // Системный вызов для отображения сообщения
    #[link_name = "_display_message"]
    pub fn display_message(ebx: u32, ecx: u32, edx: u32, esi: u32, edi: u32);

    // Системный вызов для создания кнопки
    #[link_name = "_define_button"]
    pub fn define_button(ebx: u32, ecx: u32, edx: u32, esi: u32);

    // Системный вызов для ожидания события
    #[link_name = "_wait_event"]
    pub fn wait_event() -> u32;

    // Системные вызовы для начала и конца отрисовки окна
    #[link_name = "_start_window_draw"]
    pub fn start_window_draw();

    #[link_name = "_end_window_draw"]
    pub fn end_window_draw();

    // Системный вызов для получения ID кнопки
    #[link_name = "_get_button_id"]
    pub fn get_button_id() -> u32;

    // Системный вызов для получения языка системы
    #[link_name = "_get_lang"]
    pub fn get_lang() -> u32;

    // Системный вызов для установки маски событий
    #[link_name = "_set_event_mask"]
    pub fn set_event_mask(mask: u32) -> u32;

    // Системный вызов для записи отладочной информации
    #[link_name = "_debug_write"]
    pub fn _debug_write(cl: u8);

    // Системные вызовы для управления кучей
    #[link_name = "_init_heap"]
    pub fn init_heap();

    #[link_name = "_alloc"]
    pub fn alloc(size: usize) -> *const u8;

    #[link_name = "_free"]
    pub fn free(block: *const u8) -> bool;

    // Системный вызов для загрузки динамической библиотеки
    #[link_name = "_load_dll"]
    pub fn load_dll(name: *const u8) -> *const u32;
}