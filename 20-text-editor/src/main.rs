// https://mobiarch.wordpress.com/2015/06/29/understanding-lifetime-in-rust-part-i/

pub struct TextEditor {
    text: String //Private member variable
}

impl TextEditor {
    pub fn new() -> TextEditor {
        TextEditor{ text: String::new() }
    }

    //Modify text
    pub fn add_char(&mut self, ch : char) {
        self.text.push(ch);
    }

    // Lifetime explicit but in this case it can be ellided
    pub fn get_text<'a>(&'a self) -> &'a String {
    //pub fn get_text(&self) -> &String {
        return &self.text;
    }

    pub fn reset(&mut self) {
        self.text = String::new();
    }
}

fn main() {
    let mut editor = TextEditor::new();

    editor.add_char('a');
    editor.add_char('b');
    editor.add_char('c');

    {
        let my_txt = editor.get_text();
        println!("{}", my_txt);
    }

    editor.reset();
}
