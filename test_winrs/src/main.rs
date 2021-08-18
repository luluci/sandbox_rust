use bindings::{
	Windows::Win32::UI::WindowsAndMessaging::{
		CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
		RegisterClassA, MessageBoxA, 
		MSG, WNDCLASSA, HMENU,
		WM_DESTROY, WM_PAINT,
		//WS_OVERLAPPEDWINDOW, WS_VISIBLE,
		MESSAGEBOX_STYLE,
		MB_OK,
	},
	Windows::Win32::Foundation::{
		HWND, LPARAM, WPARAM, HINSTANCE, LRESULT
	},
};

fn main() {
	unsafe {
		let caption = b"Message Box !";
		let text = b"Hello, World!";

		MessageBoxA(
			HWND(0),
			// text.as_ptr() as *const i8,
			// caption.as_ptr() as *const i8,
			"Message Box !",
			"Hello, World!",
			MB_OK
		);
	}
}
