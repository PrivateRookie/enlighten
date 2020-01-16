use cursive::traits::*;
use cursive::view::Scrollable;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, Panel, RadioGroup, TextView,
};
use cursive::Cursive;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::api::{Article, ArticleResp, Method};

#[derive(Default, Debug, Clone)]
struct MainMSG {
    article: Article,
    page: usize,
    total_page: usize,
    page_size: usize,
    index: usize,
    method: Method,
}

#[derive(Debug, Clone)]
struct RenderData {
    resp: Rc<RefCell<Option<ArticleResp>>>,
    page: Rc<Cell<usize>>,
    index: Rc<Cell<usize>>,
}

impl RenderData {
    pub fn get_art(&self, idx: usize) -> Option<Article> {
        if self.resp.borrow().is_some() {
            self.resp.borrow().clone().unwrap().data.get(idx).cloned()
        } else {
            None
        }
    }
}

pub fn render() -> impl View {
    let data = Rc::new(RenderData {
        resp: Rc::new(RefCell::new(None)),
        page: Rc::new(Cell::new(0)),
        index: Rc::new(Cell::new(0)),
    });
    let data_form = data.clone();
    let data_prev_item = data.clone();
    let data_next_item = data.clone();
    let data_prev_page = data.clone();
    let data_next_page = data;

    LinearLayout::vertical()
        .child(
            Panel::new(
                LinearLayout::horizontal()
                    .child(
                        Panel::new(
                            ListView::new()
                                .child("标题:", TextView::new("-").with_id("title"))
                                .child("作者:", TextView::new("-").with_id("writer"))
                                .child("总数:", TextView::new("-").with_id("total"))
                                .child("页数:", TextView::new("-").with_id("page_page"))
                                .child("索引:", TextView::new("-").with_id("index"))
                                .child("方法:", TextView::new("-").with_id("method")),
                        )
                        .title("信息")
                        .min_width(20)
                        .full_height(),
                    )
                    .child(
                        Panel::new(
                            TextView::empty()
                                .with_id("content")
                                .scrollable()
                                .scroll_y(true),
                        )
                        .title("正文")
                        .full_screen(),
                    ),
            )
            .full_screen(),
        )
        .child(
            Panel::new(
                LinearLayout::horizontal()
                    .child(Button::new_raw("[搜索]", move |s| {
                        render_form(s, data_form.clone())
                    }))
                    .child(TextView::new(" || "))
                    .child(Button::new_raw("[上一个]", move |s| {
                        prev_item(s, data_prev_item.clone())
                    }))
                    .child(TextView::new("|"))
                    .child(Button::new_raw("[下一个]", move |s| {
                        next_item(s, data_next_item.clone())
                    }))
                    .child(TextView::new(" || "))
                    .child(Button::new_raw("[前一页]", move |s| {
                        prev_page(s, data_prev_page.clone())
                    }))
                    .child(TextView::new("|"))
                    .child(Button::new_raw("[后一页]", move |s| {
                        next_page(s, data_next_page.clone())
                    })),
            )
            .full_width()
            .fixed_height(3),
        )
        .full_screen()
}

fn render_form(s: &mut Cursive, data: Rc<RenderData>) {
    let mut method_group: RadioGroup<Method> = RadioGroup::new();
    method_group.set_on_change(|s: &mut Cursive, v| match v {
        Method::Page => s
            .call_on_id("val", |view: &mut EditView| view.disable())
            .unwrap(),
        _ => s
            .call_on_id("val", |view: &mut EditView| view.enable())
            .unwrap(),
    });
    s.add_layer(
        Dialog::new()
            .title("输入搜索选项")
            .content(
                ListView::new()
                    .child(
                        "浏览方法",
                        LinearLayout::horizontal()
                            .child(method_group.button(Method::Page, "页数"))
                            .child(method_group.button(Method::Dynasty(String::new()), "朝代"))
                            .child(method_group.button(Method::Writer(String::new()), "作者")),
                    )
                    .child("页数", EditView::new().with_id("page").fixed_width(10))
                    .child(
                        "值",
                        EditView::new().disabled().with_id("val").fixed_width(10),
                    ),
            )
            .button("提交", move |s| on_submit(s, &method_group, data.clone()))
            .button("关闭", |s| {
                s.pop_layer();
            }),
    )
}

fn on_submit(s: &mut Cursive, m_group: &RadioGroup<Method>, data: Rc<RenderData>) {
    let method = m_group.selection();
    let page_raw = s
        .call_on_id("page", |view: &mut EditView| view.get_content())
        .unwrap();
    let val_raw = s
        .call_on_id("val", |view: &mut EditView| view.get_content())
        .unwrap();
    if let Ok(page) = page_raw.parse::<usize>() {
        let resp = match *method {
            Method::Page => ArticleResp::by_page(page),
            Method::Dynasty(_) => ArticleResp::by_dynasty(page, &val_raw.to_string()),
            Method::Writer(_) => ArticleResp::by_writer(page, &val_raw.to_string()),
        };
        match resp {
            Ok(resp) => match resp.data.get(0) {
                Some(art) => {
                    let msg = MainMSG {
                        article: art.clone(),
                        page: resp.page,
                        total_page: resp.pages,
                        page_size: resp.pagesize,
                        index: 0,
                        method: resp.method.clone(),
                    };
                    data.page.set(resp.page);
                    *data.resp.borrow_mut() = Some(resp);
                    data.index.set(1);
                    if s.cb_sink()
                        .clone()
                        .send(Box::new(move |s| update(s, msg)))
                        .is_err()
                    {
                        s.add_layer(Dialog::info("发送错误"));
                    }
                }
                None => {
                    s.add_layer(Dialog::info("内容为空!"));
                }
            },
            Err(_) => {
                s.add_layer(Dialog::info("内容获取错误"));
            }
        }
    } else {
        s.add_layer(Dialog::info("请输入正整数"));
    }
}

fn prev_item(s: &mut Cursive, data: Rc<RenderData>) {
    let idx = data.index.get();
    if idx == 0 {
        s.add_layer(Dialog::info("无更多内容"));
        return;
    }
    match data.get_art(idx - 1) {
        Some(art) => {
            let msg = MainMSG {
                article: art,
                page: data.page.get(),
                total_page: data.resp.borrow().as_ref().unwrap().total,
                page_size: data.resp.borrow().as_ref().unwrap().pagesize,
                index: idx - 1,
                method: data.resp.borrow().as_ref().unwrap().method.clone(),
            };
            s.cb_sink()
                .clone()
                .send(Box::new(|s| update(s, msg)))
                .unwrap();
            data.index.set(idx - 1);
        }
        None => s.add_layer(Dialog::info("内容为空!")),
    }
}

fn next_item(s: &mut Cursive, data: Rc<RenderData>) {
    let idx = data.index.get();
    match data.get_art(idx + 1) {
        Some(art) => {
            let msg = MainMSG {
                article: art,
                page: data.page.get(),
                total_page: data.resp.borrow().as_ref().unwrap().total,
                page_size: data.resp.borrow().as_ref().unwrap().pagesize,
                index: idx + 1,
                method: data.resp.borrow().as_ref().unwrap().method.clone(),
            };
            s.cb_sink()
                .clone()
                .send(Box::new(|s| update(s, msg)))
                .unwrap();
            data.index.set(idx + 1);
        }
        None => s.add_layer(Dialog::info("内容为空!")),
    }
}

fn prev_page(s: &mut Cursive, data: Rc<RenderData>) {
    if data.resp.borrow().is_some() {
        let art_resp = data.resp.borrow().clone().unwrap();
        let new_resp = art_resp.prev_page();
        match new_resp {
            Ok(resp) => match resp.data.get(0) {
                Some(art) => {
                    let msg = MainMSG {
                        article: art.clone(),
                        page: resp.page,
                        total_page: resp.total,
                        page_size: resp.pagesize,
                        index: 0,
                        method: resp.method.clone(),
                    };
                    s.cb_sink()
                        .clone()
                        .send(Box::new(|s| update(s, msg)))
                        .unwrap();
                    data.page.set(resp.page);
                    *data.resp.borrow_mut() = Some(resp);
                    data.index.set(0);
                }
                None => {
                    s.add_layer(Dialog::info("内容为空!"));
                }
            },
            Err(_) => {
                s.add_layer(Dialog::info("内容获取错误"));
            }
        }
    }
}

fn next_page(s: &mut Cursive, data: Rc<RenderData>) {
    if data.resp.borrow().is_some() {
        let art_resp = data.resp.borrow().clone().unwrap();
        let new_resp = art_resp.next_page();
        match new_resp {
            Ok(resp) => match resp.data.get(0) {
                Some(art) => {
                    let msg = MainMSG {
                        article: art.clone(),
                        page: resp.page,
                        total_page: resp.total,
                        page_size: resp.pagesize,
                        index: 0,
                        method: resp.method.clone(),
                    };
                    s.cb_sink()
                        .clone()
                        .send(Box::new(|s| update(s, msg)))
                        .unwrap();
                    data.page.set(resp.page);
                    *data.resp.borrow_mut() = Some(resp);
                    data.index.set(0);
                }
                None => {
                    s.add_layer(Dialog::info("内容为空!"));
                }
            },
            Err(_) => {
                s.add_layer(Dialog::info("内容获取错误"));
            }
        }
    }
}

fn update(s: &mut Cursive, msg: MainMSG) {
    s.call_on_id("title", |view: &mut TextView| {
        view.set_content(msg.article.title.to_string())
    })
    .unwrap();
    s.call_on_id("writer", |view: &mut TextView| {
        view.set_content(msg.article.writer.to_string())
    })
    .unwrap();
    s.call_on_id("total", |view: &mut TextView| {
        view.set_content(msg.total_page.to_string())
    })
    .unwrap();
    s.call_on_id("page_page", |view: &mut TextView| {
        view.set_content(msg.page.to_string())
    })
    .unwrap();
    s.call_on_id("index", |view: &mut TextView| {
        view.set_content(msg.index.to_string())
    })
    .unwrap();
    s.call_on_id("method", |view: &mut TextView| {
        let content = match msg.method.clone() {
            Method::Page => "总览".to_string(),
            Method::Dynasty(dynasty) => format!("朝代 - {}", dynasty),
            Method::Writer(writer) => format!("作者 - {}", writer),
        };
        view.set_content(content)
    })
    .unwrap();

    s.call_on_id("content", |view: &mut TextView| {
        view.set_content(msg.article.content.to_string())
    })
    .unwrap();
}
