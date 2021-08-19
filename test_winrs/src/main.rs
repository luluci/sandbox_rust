use bindings::{
	Windows::Win32::Foundation::{
		HWND, LPARAM, WPARAM, HINSTANCE, LRESULT, PWSTR
	},
	Windows::Win32::UI::WindowsAndMessaging::{
		CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
		RegisterClassW, MessageBoxW, 
		MSG, WNDCLASSW, HMENU, CW_USEDEFAULT,
		// Apis
		WM_DESTROY, WM_PAINT,
		// WINDOW_STYLE
		WS_OVERLAPPEDWINDOW, WS_VISIBLE,
		WINDOW_EX_STYLE,
		// WNDCLASS_STYLES,
		CS_HREDRAW, CS_VREDRAW,
		// MESSAGEBOX_STYLE,
		MB_OK,
		LoadCursorW, IDC_ARROW
	},
	Windows::Win32::System::LibraryLoader::{
		GetModuleHandleW
	},
	Windows::Win32::Graphics::Gdi::{
		ValidateRect,
	},
};

fn get_PWSTR(buff: &str) -> PWSTR {
	let v: Vec<u16> = buff.encode_utf16().collect();
	PWSTR(v.as_ptr() as *mut u16)
}
struct Win32WSTR {
	buff: Vec<u16>,
}
impl Win32WSTR {
	pub fn new(u8str: &String) -> Win32WSTR {
		Win32WSTR{
			buff: u8str.encode_utf16().collect(),
		}
	}

	pub fn as_pwstr(&self) -> PWSTR {
		PWSTR(self.buff.as_ptr() as *mut u16)
	}
}

fn main() {

	// https://github.com/microsoft/windows-rs/tree/0af7fb0b86a2a8f0987864da5d8244d2ddc0c34c/examples
	// https://github.com/kennykerr/samples-rs

	// Win32 App
	unsafe {
		let class_name = "MyWindowClass\0";
		let title = "Win32 App\0";
		let ws_class_name = Win32WSTR::new(&class_name.to_string());
		let ws_title = Win32WSTR::new(&title.to_string());
		let instance: HINSTANCE = GetModuleHandleW(PWSTR::NULL);
		let style = CS_HREDRAW | CS_VREDRAW;
		// WindowClass
		let mut wc = WNDCLASSW::default();
		wc.hCursor = LoadCursorW(HINSTANCE(0), IDC_ARROW);
		wc.hInstance = instance;
		wc.lpszClassName = ws_class_name.as_pwstr();
		wc.style = style;
		wc.lpfnWndProc = Some(wndproc);
		let atom: u16 = RegisterClassW(&wc);
		// CreateWindow
		let hwnd: HWND = CreateWindowExW(
			WINDOW_EX_STYLE(0),
			ws_class_name.as_pwstr(),
			ws_title.as_pwstr(),
			WS_OVERLAPPEDWINDOW | WS_VISIBLE,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			HWND(0),
			HMENU(0),
			instance,
			std::ptr::null_mut(),
		);
		// Message Loop
		let mut message = MSG::default();
		while GetMessageW(&mut message, HWND(0), 0, 0).into() {
			DispatchMessageW(&mut message);
		}
	}

	// MessageBox
	unsafe {
		let caption = "Message Box !\0";
		let text = "Hello, 世界!\0";

		//let pstr_caption = PWSTR(caption.as_ptr() as *mut u8);
		//let pstr_text = PWSTR(text.as_ptr() as *mut u8);
		let wstr_caption = Win32WSTR::new(&caption.to_string());
		let wstr_text = Win32WSTR::new(&text.to_string());

		MessageBoxW(
			HWND(0),
			// text.as_ptr() as *const i8,
			// caption.as_ptr() as *const i8,
			wstr_caption.as_pwstr(),
			wstr_text.as_pwstr(),
			MB_OK
		);
	}
}


extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	unsafe {
		match message {
			WM_PAINT => {
				println!("WM_PAINT");
				ValidateRect(window, std::ptr::null());
				LRESULT(0)
			}
			WM_DESTROY => {
				println!("WM_DESTROY");
				PostQuitMessage(0);
				LRESULT(0)
			}
			_ => DefWindowProcW(window, message, wparam, lparam),
		}
	}
}
