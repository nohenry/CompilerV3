use std::fmt::Debug;

use colored::Colorize;

pub struct TrieNode<T>
where
    T: Debug + Copy,
{
    children: [Option<Box<TrieNode<T>>>; 26],
    // is_endpoint: bool,
    keyword: Option<T>,
}

impl<T> TrieNode<T>
where
    T: Debug + Copy,
{
    pub fn new() -> Self {
        TrieNode {
            children: Default::default(),
            keyword: None,
        }
    }

    pub fn keyword(&self) -> Option<T> {
        self.keyword
    }

    fn num_children(&self) -> u8 {
        self.children.iter().fold(0, |acc, x| match x {
            Some(_) => acc + 1,
            None => acc,
        })
    }

    pub fn insert(&mut self, value: u8, keyword: Option<T>) -> &mut Box<TrieNode<T>> {
        let index = value as usize - 'a' as usize;
        let found = &mut self.children[index];
        match found {
            Some(b) => {
                b.keyword = keyword
            }
            None => {
                let b = Box::new(TrieNode {
                    children: Default::default(),
                    keyword, // is_endpoint: endpoint,
                });
                self.children[index] = Some(b);
            }
        }
        match &mut self.children[index] {
            Some(b) => b,
            None => panic!("Shouldn't be here!"),
        }
    }

    pub fn next(&self, c: char) -> Option<&TrieNode<T>> {
        match &self.children[c as usize - 'a' as usize] {
            Some(s) => Some(s.as_ref()),
            None => None,
        }
    }

    fn output(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        index: u32,
        indent: &String,
        last: bool,
        value: i8,
    ) -> std::fmt::Result {
        write!(f, "{}", indent)?;
        if index != 0 {
            write!(f, "{}", if last { "└──" } else { "├──" })?;
        }
        if let Some(k) = &self.keyword {
            if value >= 0 {
                write!(
                    f,
                    "{} ::",
                    format!("{}", (value as u8 + 'a' as u8) as char).bright_green(),
                )?;
                k.fmt(f)?;
                write!(f, "\n")?;
            }
        } else if value != -1 {
            write!(f, "{}\n", (value as u8 + 'a' as u8) as char)?;
        }
        let nindent = format!(
            "{}{}",
            indent,
            if index == 0 {
                ""
            } else if last {
                "    "
            } else {
                "│   "
            }
        );
        let mut num = 0;
        self.children.iter().enumerate().for_each(|(i, v)| match v {
            Some(v) => {
                v.output(
                    f,
                    index + 1,
                    &nindent,
                    num == self.num_children() - 1,
                    i as i8,
                )
                .unwrap();
                num += 1;
            }
            None => (),
        });
        Ok(())
    }
}

impl<T> Debug for TrieNode<T>
where
    T: Debug + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.output(f, 0, &"".to_string(), false, -1)
    }
}
