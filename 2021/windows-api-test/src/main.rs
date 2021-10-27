use windows::UI::Color;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
   let color=Color{
       A:1,
       R:3,
       G:2,
       B:1,
   };
   println!("{:?}",color);
   unsafe{
      MessageBoxA(None, "Text", "Caption", MB_OK);
   }
   
}
