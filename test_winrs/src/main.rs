use std::{borrow::BorrowMut, ffi::c_void};

use bindings::{
	Windows::Win32::Foundation::{
		HWND, LPARAM, WPARAM, HINSTANCE, LRESULT, PWSTR
	},
	Windows::Win32::UI::WindowsAndMessaging::{
		GetMessageW, PeekMessageW,
		CreateWindowExW, DefWindowProcW, DispatchMessageW, PostQuitMessage,
		RegisterClassW, MessageBoxW, TranslateMessage, 
		MSG, WNDCLASSW, HMENU, CREATESTRUCTW, CW_USEDEFAULT,
		// Apis
		WM_DESTROY, WM_PAINT, WM_CREATE, WM_QUIT, 
		WINDOW_STYLE,
		WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_CHILDWINDOW, WS_CHILD, WS_BORDER,
		ES_AUTOHSCROLL, ES_MULTILINE, 
		WINDOW_EX_STYLE,
		WS_EX_ACCEPTFILES,
		// WNDCLASS_STYLES,
		CS_HREDRAW, CS_VREDRAW,
		// MESSAGEBOX_STYLE,
		MB_OK,
		LoadCursorW, IDC_ARROW,
		PEEK_MESSAGE_REMOVE_TYPE,
		PM_REMOVE, 
	},
	Windows::Win32::UI::Controls::{
		InitCommonControlsEx,
		INITCOMMONCONTROLSEX, INITCOMMONCONTROLSEX_ICC
	},
	Windows::Win32::System::LibraryLoader::{
		GetModuleHandleW
	},
	Windows::Win32::Graphics::Gdi::{
		ValidateRect, GetStockObject, UpdateWindow,
		HBRUSH, GET_STOCK_OBJECT_FLAGS,
		DKGRAY_BRUSH,
		HGDIOBJ,
	},
};

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

struct WndCoord {
	x: i32,
	y: i32,
	w: i32,
	h: i32,
	child: Vec<Self>,
}
impl WndCoord {
	const PADDING: i32 = 5;

	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		WndCoord{
			x,
			y,
			w: width,
			h: height,
			child: vec![]
		}
	}

	pub fn add_child(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
		// PADDINGチェック
		if x < Self::PADDING {
			x = Self::PADDING;
		}
		if y < Self::PADDING {
			y = Self::PADDING;
		}
		if (x + width) > (self.w - Self::PADDING) {
			width = self.w - Self::PADDING;
		}
		if (y + height) > (self.h - Self::PADDING) {
			height = self.h - Self::PADDING;
		}
		// 登録
		let child :WndCoord = WndCoord::new(x,y,width,height);
		self.child.push(child);
	}

	pub fn get_child(&self, idx: usize) -> &WndCoord {
		let child = self.child.get(idx);
		match child {
			Some(_child) => _child,
			None => panic!("invalid index!")
		}
	}

	pub fn x(&self) -> i32 {
		self.x
	}
	pub fn y(&self) -> i32 {
		self.y
	}
	pub fn width(&self) -> i32 {
		self.w
	}
	pub fn height(&self) -> i32 {
		self.h
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
		let brush: HGDIOBJ = GetStockObject(DKGRAY_BRUSH);
		//wc.hbrBackground = *(&brush as *const HGDIOBJ as *const HBRUSH);
		//wc.hbrBackground = HBRUSH(GetStockObject(DKGRAY_BRUSH).0);
		wc.hbrBackground = HBRUSH(brush.0);
		let atom: u16 = RegisterClassW(&wc);
		// 座標調整オブジェクト作成
		let mut coord = WndCoord::new(0,0, 800, 200);
		coord.add_child(5, 50, 800, 200);
		// CreateWindow
		let hwnd_root: HWND = CreateWindowExW(
			WS_EX_ACCEPTFILES,
			ws_class_name.as_pwstr(),
			ws_title.as_pwstr(),
			WS_OVERLAPPEDWINDOW | WS_VISIBLE,
			CW_USEDEFAULT,
			CW_USEDEFAULT,
			coord.width(),
			coord.height(),
			HWND(0),
			HMENU(0),
			instance,
			std::ptr::null_mut(),
		);
		UpdateWindow(hwnd_root);

		// Message Loop
		let mut message = MSG::default();
		// while message.message != WM_QUIT {
		// 	if PeekMessageW(&mut message, HWND(0), 0, 0, PM_REMOVE).into() {
		// 		TranslateMessage(&mut message);
		// 		DispatchMessageW(&mut message);
		// 	}
		// }
		// if message.message == WM_QUIT {
		// 	println!("WM_QUIT");
		// 	PostQuitMessage(0);
		// }
		while GetMessageW(&mut message, HWND(0), 0, 0).into() {
			TranslateMessage(&mut message);
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
			WM_CREATE => {
				wndproc_wm_create(window, message, wparam, lparam)
			},
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

fn wndproc_wm_create(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	unsafe {
		//let cs: CREATESTRUCTW = CREATESTRUCTW::into::<LPARAM>(lparam);
		let cs = lparam.0 as *const CREATESTRUCTW;

		// コモンコントロール初期化
		let icc = INITCOMMONCONTROLSEX::default();
		InitCommonControlsEx(&icc);

		//
		let mut ws_val = 0;
		ws_val |= (WS_CHILD | WS_VISIBLE | WS_BORDER).0;
		ws_val |= (ES_AUTOHSCROLL | ES_MULTILINE) as u32;
		let ws = WINDOW_STYLE(ws_val);
		let edit_class_name = "EDIT\0";
		let init_text = "test\0";
		let ws_edit_class_name = Win32WSTR::new(&edit_class_name.to_string());
		let ws_init_text = Win32WSTR::new(&init_text.to_string());
		let hwnd_edit: HWND = CreateWindowExW(
			WS_EX_ACCEPTFILES,
			ws_edit_class_name.as_pwstr(),
			ws_init_text.as_pwstr(),
			ws,
			// edit_coord.x(),
			// edit_coord.y(),
			// edit_coord.width(),
			// edit_coord.height(),
			5,
			5,
			500,
			100,
			window,
			HMENU(0),
			(*cs).hInstance,
			std::ptr::null_mut(),
		);
	}

	LRESULT(0)
}
