如果你想让用户通过图形用户界面（GUI）选择两张图片来进行比较，而不是在代码中硬编码图片路径，你可以使用像gtk-rs（GTK+的Rust绑定）这样的GUI库来创建一个简单的文件选择器对话框。以下是一个简化的步骤和示例代码，展示了如何使用gtk-rs和opencv-rust来实现这一功能：

步骤
安装必要的库：
通过Cargo安装gtk和opencv。

bash
cargo add gtk opencv
注意：你还需要在系统上安装GTK+库和OpenCV库。

创建GUI：
使用gtk-rs创建一个简单的窗口，其中包含两个按钮用于选择图片，以及一个显示结果的区域（如GtkImage）。

实现文件选择逻辑：
为两个按钮添加点击事件处理程序，当点击时打开一个文件选择对话框让用户选择图片文件。

图片比较和显示：
在用户选择图片后，使用opencv-rust比较图片并标记差异。然后，将结果图片显示在GUI中。

示例代码
以下是一个简化的示例代码，展示了如何使用gtk-rs和opencv-rust来实现上述功能：

rust
extern crate gtk;  
#[macro_use]  
extern crate opencv;  
  
use gtk::prelude::*;  
use gtk::{Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog, Image, Widget};  
use opencv::core;  
use opencv::imgcodecs;  
use opencv::imgproc;  
use opencv::types::Mat;  
use std::error::Error;  
use std::path::Path;  
  
fn compare_images(path1: &str, path2: &str) -> Result<Mat, Box<dyn Error>> {  
    // ... 省略图片加载、比较和差异计算的代码 ...  
    // 假设这里返回了一个包含差异信息的Mat对象  
    Ok(Mat::zeros(core::Size::new(100, 100), core::CV_8UC3)) // 示例：返回一个100x100的黑色图像  
}  
  
fn on_button_clicked(button: &Button, file_chooser_type: FileChooserAction, image_widget: &Image) {  
    let dialog = FileChooserDialog::new(  
        Some("Open File"),  
        None::<&ApplicationWindow>,  
        file_chooser_type,  
        &["Cancel", "Open"],  
    );  
  
    dialog.set_secondary_text("Please choose a file");  
  
    if let Some(response) = dialog.run() {  
        if response == gtk::ResponseType::Accept {  
            if let Some(filename) = dialog.get_filename() {  
                // 这里只是打印文件名，你应该在这里调用compare_images函数并更新image_widget  
                println!("Selected file: {}", filename.display());  
                // 假设你已经有了差异图片的Mat对象，你可以将其转换为GtkImage可以显示的格式  
                // ...  
                // image_widget.set_from_pixbuf(...); // 更新GUI中的图片显示  
            }  
        }  
    }  
  
    dialog.close();  
}  
  
fn main() -> Result<(), Box<dyn Error>> {  
    gtk::init()?;  
  
    let app = Application::new("org.gtk-rs.example", Default::default())?;  
  
    let window = ApplicationWindow::new(app.clone());  
    window.set_title("Image Comparison");  
    window.set_default_size(400, 300);  
  
    let button1 = Button::with_label("Select Image 1");  
    button1.connect_clicked(clone!(@weak window => move |_| {  
        on_button_clicked(&button1, FileChooserAction::Open, &window.child_by_type<Image>().unwrap());  
    }));  
  
    let button2 = Button::with_label("Select Image 2");  
    // 你可以为第二个按钮添加类似的处理程序，但这里只是示例，所以省略了  
  
    let image = Image::new(); // 这里应该是一个可以显示差异图片的Image控件  
  
    window.add(&gtk::Box::new(gtk::Orientation::Vertical, 5)  
        .pack_start(&button1, true, true, 0)  
        .pack_start(&button2, true, true, 0)  
        .pack_start(&image, true, true, 0)  
        .cast_into::<Widget>()?  
    );  
  
    window.show
}