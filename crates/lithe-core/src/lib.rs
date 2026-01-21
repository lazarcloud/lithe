pub trait Component {
    fn render(&self, buf: &mut String);
}

pub fn render_to_string<C: Component>(component: &C) -> String {
    let mut buf = String::new();
    component.render(&mut buf);
    buf
}

impl Component for String {
    fn render(&self, buf: &mut String) {
        buf.push_str(self);
    }
}

impl Component for &str {
    fn render(&self, buf: &mut String) {
        buf.push_str(self);
    }
}

#[cfg(test)]
mod tests;
