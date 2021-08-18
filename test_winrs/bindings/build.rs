fn main() {
	// https://github.com/microsoft/windows-rs
	// https://microsoft.github.io/windows-docs-rs/doc/bindings/Windows/
	// https://crates.io/crates/windows
	windows::build!(
		Windows::Win32::UI::WindowsAndMessaging::{
			CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
			RegisterClassA, MessageBoxA, 
			MSG, WNDCLASSA, HMENU,
			WM_DESTROY, WM_PAINT,
			//WS_OVERLAPPEDWINDOW, WS_VISIBLE,
		},
		Windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE,			// include: MB_OK
		// Windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE::{
		// 	MB_OK
		// },
		Windows::Win32::Foundation::{
			HWND, LPARAM, WPARAM, HINSTANCE, LRESULT
		},

	);
}