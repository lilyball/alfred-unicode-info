/// Helpers for writing Alfred XML output

use std::io;
use std::io::BufferedWriter;
use std::str;

/// Representation of an <item>
#[deriving(Eq,Clone)]
pub struct Item {
    /// Identifier for the results. If given, must be unique among items, and is used for
    /// prioritizing feedback results based on usage. If blank, Alfred uses a UUID and does
    /// not learn from the results.
    uid: Option<str::MaybeOwned<'static>>,
    /// The value that is passed to the next portion of the workflow when this item
    /// is selected.
    arg: Option<str::MaybeOwned<'static>>,
    /// What type of result this is, if any.
    type_: Option<ItemType>,
    /// Whether or not the result item is 'valid'. If `false`, `autocomplete` may be used.
    valid: bool,
    /// Autocomplete data for valid=false items. When this item is selected, the autocomplete
    /// value is inserted into the Alfred window.
    autocomplete: Option<str::MaybeOwned<'static>>,

    /// Title for the item
    title: str::MaybeOwned<'static>,
    /// Subtitle for the item
    subtitle: Option<str::MaybeOwned<'static>>,
    /// Icon for the item
    icon: Option<Icon>
}

impl Item {
    /// Returns a new Item with the given title
    pub fn new<S: str::IntoMaybeOwned<'static>>(title: S) -> Item {
        Item {
            uid: None,
            arg: None,
            type_: None,
            valid: true,
            autocomplete: None,
            title: title.into_maybe_owned(),
            subtitle: None,
            icon: None
        }
    }
}

/// Item icons
#[deriving(Eq,Clone)]
pub enum Icon {
    /// Path to an image file on disk relative to the workflow directory
    PathIcon(str::MaybeOwned<'static>),
    /// Path to a file whose icon will be used
    FileIcon(str::MaybeOwned<'static>),
    /// UTI for a file type to use (e.g. public.folder)
    FileType(str::MaybeOwned<'static>)
}

/// Item types
#[deriving(Eq,Clone)]
pub enum ItemType {
    FileItemType
}

impl Item {
    /// Writes the XML fragment representing the Item to the Writer
    pub fn write_xml(&self, w: &mut io::Writer, indent: uint) -> io::IoResult<()> {
        fn write_indent(w: &mut io::Writer, indent: uint) -> io::IoResult<()> {
            for _ in range(0, indent) {
                if_ok!(w.write_str("    "));
            }
            Ok(())
        }

        let mut w = BufferedWriter::with_capacity(512, w);

        if_ok!(write_indent(&mut w, indent));
        if_ok!(w.write_str("<item"));
        match self.uid {
            None => (),
            Some(ref uid) => {
                if_ok!(write!(&mut w, r#" uid="{}""#, encode_entities(uid.as_slice())));
            }
        }
        match self.arg {
            None => (),
            Some(ref arg) => {
                if_ok!(write!(&mut w, r#" arg="{}""#, encode_entities(arg.as_slice())));
            }
        }
        match self.type_ {
            None => (),
            Some(FileItemType) => {
                if_ok!(w.write_str(r#" type="file""#));
            }
        }
        if_ok!(write!(&mut w, r#" valid="{}""#, if self.valid { "yes" } else { "no" }));
        match self.autocomplete {
            None => (),
            Some(ref auto) => {
                if_ok!(write!(&mut w, r#" autocomplete="{}""#, encode_entities(auto.as_slice())));
            }
        }
        if_ok!(w.write_str(">\n"));

        if_ok!(write_indent(&mut w, indent+1));
        if_ok!(write!(&mut w, "<title>{}</title>\n", encode_entities(self.title.as_slice())));

        match self.subtitle {
            None => (),
            Some(ref s) => {
                if_ok!(write_indent(&mut w, indent+1));
                if_ok!(write!(&mut w, "<subtitle>{}</subtitle>\n", encode_entities(s.as_slice())));
            }
        }

        match self.icon {
            None => (),
            Some(ref icon) => {
                if_ok!(write_indent(&mut w, indent+1));
                match *icon {
                    PathIcon(ref s) => {
                        if_ok!(write!(&mut w, "<icon>{}</icon>\n", encode_entities(s.as_slice())));
                    }
                    FileIcon(ref s) => {
                        if_ok!(write!(&mut w, "<icon type=\"fileicon\">{}</icon>\n",
                                      encode_entities(s.as_slice())));
                    }
                    FileType(ref s) => {
                        if_ok!(write!(&mut w, "<icon type=\"filetype\">{}</icon>\n",
                                      encode_entities(s.as_slice())));
                    }
                }
            }
        }

        if_ok!(write_indent(&mut w, indent));
        if_ok!(w.write_str("</item>\n"));

        if_ok!(w.flush());
        Ok(())
    }
}

fn encode_entities<'a>(s: &'a str) -> str::MaybeOwned<'a> {
    fn encode_entity(c: char) -> Option<&'static str> {
        Some(match c {
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '&' => "&amp;",
            '\0'..'\x1F' => {
                // control characters can't be decoded by NSXMLParser. I'm unsure if this is
                // an actual restriction in XML or just libxml2, but it holds even if they're
                // encoded as entities. Therefore, use the replacement character.
                "\uFFFD"
            }
            _ => return None
        })
    }

    if s.chars().any(|c| encode_entity(c).is_some()) {
        let mut res = str::with_capacity(s.len());
        for c in s.chars() {
            match encode_entity(c) {
                Some(ent) => res.push_str(ent),
                None => res.push_char(c)
            }
        }
        str::Owned(res)
    } else {
        str::Slice(s)
    }
}
