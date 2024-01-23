// TODO: implement
#![allow(unused)]

use std::cell::{Cell, RefCell};
use std::option::Option;
use std::rc::{Rc, Weak};
use std::vec::Vec;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::widgets::{Block, Widget};

use crate::components::Component;

pub struct DomNode<W>
where
    W: Widget,
{
    pub parent: Weak<RefCell<DomNode<W>>>,
    pub component: Option<Box<dyn Component<W>>>,
    pub container: Cell<Rect>,
    pub children: RefCell<Vec<Rc<RefCell<DomNode<W>>>>>,
}

impl<W> DomNode<W>
where
    W: Widget,
{
    pub fn new(
        parent: Weak<RefCell<DomNode<W>>>,
        component: Option<Box<dyn Component<W>>>,
        area: Rect,
    ) -> Self {
        DomNode {
            parent,
            component,
            container: Cell::new(area),
            children: RefCell::new(Vec::new()),
        }
    }

    pub fn add_child(
        parent_rc: Rc<RefCell<DomNode<W>>>,
        component: Box<dyn Component<W>>,
        container: Option<Rect>,
    ) -> Rc<RefCell<DomNode<W>>> {
        let child = Rc::new(RefCell::new(DomNode::new(
            Rc::downgrade(&parent_rc),
            Some(component),
            container.unwrap_or_default(),
        )));
        let parent = parent_rc.borrow_mut();
        parent.children.borrow_mut().push(child.clone());

        child
    }

    pub fn is_leaf(&self) -> bool {
        self.children.borrow().len() == 0
    }

    pub fn is_root(&self) -> bool {
        self.parent.upgrade().is_none()
    }
}

pub struct Dom {
    pub root: Rc<RefCell<DomNode<Block<'static>>>>,
}

impl Default for Dom {
    fn default() -> Self {
        Dom {
            root: Rc::new(RefCell::new(DomNode {
                parent: Weak::new(),
                component: Option::None,
                container: Cell::default(),
                children: RefCell::new(Vec::new()),
            })),
        }
    }
}

impl Dom {
    pub fn render<B, W>(frame: &mut Frame<B>, root: &Rc<RefCell<DomNode<W>>>)
    where
        B: Backend,
        W: Widget,
    {
        let mut queue: Vec<Rc<RefCell<DomNode<W>>>> = Vec::new();
        queue.push(root.clone());
        while !queue.is_empty() {
            let rcnode = queue.pop();
            if rcnode.is_none() {
                continue;
            }
            let c = rcnode.as_deref();
            if c.is_none() {
                continue;
            }
            let node = c.expect("RC DomNode deleted").borrow();
            if let Some(c) = node.component.as_deref() {
                frame.render_widget(
                    c.widget(),
                    c.area(
                        node.parent
                            .upgrade()
                            .as_deref()
                            .unwrap()
                            .borrow()
                            .container
                            .get(),
                    ),
                );
            }

            queue.append(&mut node.children.borrow_mut());
        }
    }
}
