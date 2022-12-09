// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::LPVOID;
use shared::windef::HWND;
use um::oleidl::LPDROPTARGET;
use um::winnt::HRESULT;
extern "system" {
    pub fn OleInitialize(
        pvReserved: LPVOID,
    ) -> HRESULT;
    pub fn RegisterDragDrop(
        hwnd: HWND,
        pDropTarget: LPDROPTARGET,
    ) -> HRESULT;
    pub fn RevokeDragDrop(
        hwnd: HWND,
    ) -> HRESULT;
}
