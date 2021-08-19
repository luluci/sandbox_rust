fn main() {
	// https://github.com/microsoft/windows-rs
	// https://microsoft.github.io/windows-docs-rs/doc/bindings/Windows/
	// https://crates.io/crates/windows
	windows::build!(
		Windows::Win32::UI::WindowsAndMessaging::{
			CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
			RegisterClassW, MessageBoxW,
			MSG, WNDCLASSW, HMENU,
			CW_USEDEFAULT,
			WM_DESTROY, WM_PAINT,
			LoadCursorW, IDC_ARROW,
			WINDOW_STYLE, WINDOW_EX_STYLE,
			WNDCLASS_STYLES,
			MESSAGEBOX_STYLE,			// include: MB_OK
		},
		Windows::Win32::System::LibraryLoader::{
			GetModuleHandleW
		},
		Windows::Win32::Foundation::{
			HWND, LPARAM, WPARAM, HINSTANCE, LRESULT, PWSTR
		},
		Windows::Win32::Graphics::Gdi::{
			ValidateRect,
		},
	);
}