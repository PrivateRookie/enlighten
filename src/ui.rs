use cursive::traits::*;
use cursive::view::Scrollable;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LayerPosition, LinearLayout, ListView, Panel, RadioGroup,
    SelectView, StackView, TextView,
};
use cursive::Cursive;
use rand::Rng;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::api::*;

const CN_PUNCTIONS: [char; 74] = [
    '！', '？', '｡', '＂', '＃', '＄', '％', '＆', '＇', '（', '）', '＊', '＋', '，', '－', '／',
    '：', '；', '＜', '＝', '＞', '＠', '［', '＼', '］', '＾', '＿', '｀', '｛', '｜', '｝', '～',
    '｟', '｠', '｢', '｣', '､', '、', '〃', '》', '「', '」', '『', '』', '【', '】', '〔', '〕',
    '〖', '〗', '〘', '〙', '〚', '〛', '〜', '〝', '〞', '〟', '〰', '〾', '〿', '–', '—', '‘',
    '’', '‛', '“', '”', '„', '‟', '…', '‧', '﹏', '.',
];

#[derive(Default, Debug, Clone)]
struct MSG {
    article: Article,
    page: usize,
    total: usize,
    page_size: usize,
    index: usize,
    method: Method,
}

enum MaskLevel {
    Empty,
    Light,
    Medium,
    Heavy,
    Full,
}

impl Into<String> for MaskLevel {
    fn into(self) -> String {
        match self {
            MaskLevel::Empty => "无".to_string(),
            MaskLevel::Light => "轻".to_string(),
            MaskLevel::Medium => "中".to_string(),
            MaskLevel::Heavy => "重".to_string(),
            MaskLevel::Full => "全".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct RenderData {
    art_resp: Rc<RefCell<Option<ArtListResp>>>,
    sen_resp: Rc<RefCell<Option<SentenceListResp>>>,
    writer_resp: Rc<RefCell<Option<WriterListResp>>>,
    index: Rc<Cell<usize>>,
    sview_vec: RefCell<Vec<String>>,
}

impl RenderData {
    fn new() -> RenderData {
        RenderData {
            art_resp: Rc::new(RefCell::new(None)),
            sen_resp: Rc::new(RefCell::new(None)),
            writer_resp: Rc::new(RefCell::new(None)),
            index: Rc::new(Cell::new(0)),
            sview_vec: RefCell::new("r t s c".split(' ').map(|i| i.to_string()).collect()),
        }
    }

    fn get_artitle(&self, idx: usize) -> Option<Article> {
        if self.art_resp.borrow().is_some() {
            match self.art_resp.borrow().as_ref().unwrap().data.get(idx) {
                Some(art) => art.show().ok(),
                None => None,
            }
        } else {
            None
        }
    }
}

pub fn render() -> impl View {
    let data = Rc::new(RenderData::new());
    let data_content = data.clone();
    let data_remark = data.clone();
    let data_trans = data.clone();
    let data_sx = data.clone();
    let data_mask = data.clone();
    let data_form = data.clone();
    let data_prev_item = data.clone();
    let data_next_item = data.clone();
    let data_prev_page = data.clone();
    let data_next_page = data;

    let mut stack_view = StackView::new();

    stack_view.add_fullscreen_layer(
        Panel::new(
            TextView::empty()
                .with_id("remark_text")
                .scrollable()
                .scroll_y(true),
        )
        .title("注释")
        .full_screen(),
    );
    stack_view.add_fullscreen_layer(
        Panel::new(
            TextView::empty()
                .with_id("translation_text")
                .scrollable()
                .scroll_y(true),
        )
        .title("翻译")
        .full_screen(),
    );
    stack_view.add_fullscreen_layer(
        Panel::new(
            TextView::empty()
                .with_id("shangxi_text")
                .scrollable()
                .scroll_y(true),
        )
        .title("赏析")
        .full_screen(),
    );
    stack_view.add_fullscreen_layer(
        Panel::new(
            TextView::empty()
                .with_id("content_text")
                .scrollable()
                .scroll_y(true),
        )
        .title("正文")
        .full_screen(),
    );

    LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(
                    Panel::new(
                        ListView::new()
                            .child("标题:", TextView::new("-").with_id("title"))
                            .child("作者:", TextView::new("-").with_id("writer"))
                            .child(
                                "正文:",
                                LinearLayout::horizontal()
                                    .child(
                                        Button::new_raw("-", move |s| {
                                            visible_view(s, "c", data_content.clone())
                                        })
                                        .disabled()
                                        .with_id("content_btn"),
                                    )
                                    .child(DummyView.full_width()),
                            )
                            .child(
                                "注释:",
                                LinearLayout::horizontal()
                                    .child(
                                        Button::new_raw("-", move |s| {
                                            visible_view(s, "r", data_remark.clone())
                                        })
                                        .disabled()
                                        .with_id("remark_btn"),
                                    )
                                    .child(DummyView.full_width()),
                            )
                            .child(
                                "翻译:",
                                LinearLayout::horizontal()
                                    .child(
                                        Button::new_raw("-", move |s| {
                                            visible_view(s, "t", data_trans.clone())
                                        })
                                        .disabled()
                                        .with_id("translation_btn"),
                                    )
                                    .child(DummyView.full_width()),
                            )
                            .child(
                                "赏析:",
                                LinearLayout::horizontal()
                                    .child(
                                        Button::new_raw("-", move |s| {
                                            visible_view(s, "s", data_sx.clone())
                                        })
                                        .disabled()
                                        .with_id("shangxi_btn"),
                                    )
                                    .child(DummyView.full_width()),
                            )
                            .child("总数:", TextView::new("-").with_id("total"))
                            .child("页数:", TextView::new("-").with_id("page_page"))
                            .child("索引:", TextView::new("-").with_id("index"))
                            .child("方法:", TextView::new("-").with_id("method")),
                    )
                    .title("信息")
                    .min_width(20)
                    .full_height(),
                )
                .child(stack_view.with_id("stack_view").min_width(80)),
        )
        .child(
            Panel::new(
                LinearLayout::horizontal()
                    .child(DummyView.full_width())
                    .child(
                        Button::new_raw("[ 搜索 ]", move |s| render_form(s, data_form.clone()))
                            .with_id("search_button"),
                    )
                    .child(Button::new_raw("[ 背诵 ]", move |s| {
                        let d = data_mask.clone();
                        let mut select = SelectView::new().autojump();
                        select.add_item("无", MaskLevel::Empty);
                        select.add_item("轻", MaskLevel::Light);
                        select.add_item("中", MaskLevel::Medium);
                        select.add_item("重", MaskLevel::Heavy);
                        select.add_item("全", MaskLevel::Full);
                        select.set_on_submit(move |s: &mut Cursive, val: &MaskLevel| {
                            mask_content(s, d.clone(), val)
                        });
                        s.add_layer(Dialog::around(select).button("关闭", |s| {
                            s.pop_layer();
                        }))
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
                    .child(Button::new_raw("[下一页]", move |s| {
                        next_page(s, data_next_page.clone())
                    })),
            )
            .full_width()
            .fixed_height(3),
        )
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
                        "方法",
                        LinearLayout::horizontal()
                            .child(method_group.button(Method::Page, "页数"))
                            .child(method_group.button(Method::Writer(String::new()), "作者"))
                            .child(method_group.button(Method::Keyword(String::new()), "关键字"))
                            .child(method_group.button(Method::Dynasty(String::new()), "朝代")),
                    )
                    .child(
                        "页数",
                        EditView::new().content("1").with_id("page").fixed_width(10),
                    )
                    .child(
                        "输入",
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
        if page < 1 {
            s.add_layer(Dialog::info("请输入正整数( >= 1)"));
            return;
        }
        let resp = match method.as_ref() {
            Method::Page => ArtListResp::list_by_page(page),
            Method::Dynasty(_) => ArtListResp::list_by_dynasty(page, val_raw.to_string()),
            Method::Writer(_) => ArtListResp::list_by_writer(page, val_raw.to_string()),
            Method::Keyword(_) => ArtListResp::list_by_keyword(page, val_raw.to_string()),
        };

        match resp {
            Ok(resp) => match resp.data.get(0) {
                Some(art) => match art.show() {
                    Ok(article) => {
                        let msg = MSG {
                            article,
                            page: resp.page,
                            total: resp.total,
                            page_size: resp.page_size,
                            index: 0,
                            method: method.as_ref().clone(),
                        };
                        *data.art_resp.borrow_mut() = Some(resp);
                        data.index.set(0);
                        if s.cb_sink()
                            .clone()
                            .send(Box::new(move |s| update(s, msg)))
                            .is_err()
                        {
                            s.add_layer(Dialog::info("发送错误"));
                        }
                    }
                    Err(_) => {
                        s.add_layer(Dialog::info("内容获取错误"));
                    }
                },
                None => {
                    s.add_layer(Dialog::info("内容为空!"));
                }
            },
            Err(_) => {
                s.add_layer(Dialog::info("内容获取错误"));
            }
        }
    } else {
        s.add_layer(Dialog::info("请输入正整数( >= 1)"));
    }
}

fn prev_item(s: &mut Cursive, data: Rc<RenderData>) {
    let idx = data.index.get();
    if idx == 0 {
        s.add_layer(Dialog::info("无更多内容"));
        return;
    }
    match data.get_artitle(idx - 1) {
        Some(art) => {
            let msg = MSG {
                article: art,
                total: data.art_resp.borrow().as_ref().unwrap().total,
                page: data.art_resp.borrow().as_ref().unwrap().page,
                page_size: data.art_resp.borrow().as_ref().unwrap().page_size,
                index: idx - 1,
                method: data.art_resp.borrow().as_ref().unwrap().method.clone(),
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
    match data.get_artitle(idx + 1) {
        Some(art) => {
            let msg = MSG {
                article: art,
                total: data.art_resp.borrow().as_ref().unwrap().total,
                page: data.art_resp.borrow().as_ref().unwrap().page,
                page_size: data.art_resp.borrow().as_ref().unwrap().page_size,
                index: idx + 1,
                method: data.art_resp.borrow().as_ref().unwrap().method.clone(),
            };
            s.cb_sink()
                .clone()
                .send(Box::new(|s| update(s, msg)))
                .expect("发送错误");
            data.index.set(idx + 1);
        }
        None => s.add_layer(Dialog::info("内容为空!")),
    }
}

fn prev_page(s: &mut Cursive, data: Rc<RenderData>) {
    if data.art_resp.borrow().is_some() {
        let art_resp = data.art_resp.borrow().clone().unwrap();
        let new_resp = art_resp.prev_page();
        match new_resp {
            Ok(resp) => match resp.data.get(0) {
                Some(art) => match art.show() {
                    Ok(article) => {
                        let msg = MSG {
                            article,
                            page: resp.page,
                            total: resp.total,
                            page_size: resp.page_size,
                            index: 0,
                            method: art_resp.method.clone(),
                        };
                        *data.art_resp.borrow_mut() = Some(resp);
                        data.index.set(0);
                        if s.cb_sink()
                            .clone()
                            .send(Box::new(move |s| update(s, msg)))
                            .is_err()
                        {
                            s.add_layer(Dialog::info("发送错误"));
                        }
                    }
                    Err(_) => {
                        s.add_layer(Dialog::info("内容获取错误"));
                    }
                },
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
    if data.art_resp.borrow().is_some() {
        let art_resp = data.art_resp.borrow().clone().unwrap();
        let new_resp = art_resp.next_page();
        match new_resp {
            Ok(resp) => match resp.data.get(0) {
                Some(art) => match art.show() {
                    Ok(article) => {
                        let msg = MSG {
                            article,
                            page: resp.page,
                            total: resp.total,
                            page_size: resp.page_size,
                            index: 0,
                            method: art_resp.method.clone(),
                        };
                        *data.art_resp.borrow_mut() = Some(resp);
                        data.index.set(0);
                        if s.cb_sink()
                            .clone()
                            .send(Box::new(move |s| update(s, msg)))
                            .is_err()
                        {
                            s.add_layer(Dialog::info("发送错误"));
                        }
                    }
                    Err(_) => {
                        s.add_layer(Dialog::info("内容获取错误"));
                    }
                },
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

fn update(s: &mut Cursive, msg: MSG) {
    fn render_label(s: &mut Cursive, id: &str, msg: &MSG) {
        s.call_on_id(id, |view: &mut Button| {
            if let Some(_) = msg.article.remark {
                view.enable();
                view.set_label_raw("[ √ ]");
            } else {
                view.set_label_raw("[ × ]")
            }
        })
        .unwrap();
    }

    s.call_on_id("title", |view: &mut TextView| {
        view.set_content(msg.article.title.to_string())
    })
    .unwrap();
    s.call_on_id("writer", |view: &mut TextView| {
        view.set_content(msg.article.writer.to_string())
    })
    .unwrap();
    render_label(s, "content_btn", &msg);
    render_label(s, "remark_btn", &msg);
    render_label(s, "translation_btn", &msg);
    render_label(s, "shangxi_btn", &msg);
    s.call_on_id("total", |view: &mut TextView| {
        view.set_content(msg.total.to_string())
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
            Method::Keyword(keyword) => format!("关键字 - {}", keyword),
        };
        view.set_content(content)
    })
    .unwrap();

    s.call_on_id("content_text", |view: &mut TextView| {
        view.set_content(msg.article.content.to_string())
    })
    .unwrap();

    s.call_on_id("remark_text", |view: &mut TextView| {
        view.set_content(msg.article.remark.clone().unwrap())
    })
    .unwrap();

    s.call_on_id("translation_text", |view: &mut TextView| {
        view.set_content(msg.article.translation.clone().unwrap())
    })
    .unwrap();

    s.call_on_id("shangxi_text", |view: &mut TextView| {
        view.set_content(msg.article.shangxi.clone().unwrap())
    })
    .unwrap();
}

fn visible_view(s: &mut Cursive, id: &str, data: Rc<RenderData>) {
    s.call_on_id("stack_view", |view: &mut StackView| {
        let pos = data
            .sview_vec
            .borrow()
            .iter()
            .position(|i| i == id)
            .unwrap();
        let e = data.sview_vec.borrow_mut().remove(pos);
        data.sview_vec.borrow_mut().push(e);
        view.move_to_front(LayerPosition::FromBack(pos));
    })
    .unwrap();
}

fn mask_content(s: &mut Cursive, data: Rc<RenderData>, level: &MaskLevel) {
    let mut rng = rand::thread_rng();
    let level = match level {
        MaskLevel::Empty => 0.0,
        MaskLevel::Light => 30.0,
        MaskLevel::Medium => 60.0,
        MaskLevel::Heavy => 80.0,
        MaskLevel::Full => 100.0,
    };
    let art = data.get_artitle(data.index.get());
    if let Some(art) = art {
        let masked_content: String = art
            .content
            .chars()
            .map(|c: char| {
                if !CN_PUNCTIONS.contains(&c) && !c.is_control() && !c.is_whitespace() {
                    if (rng.gen::<f32>() * 100.0) < level {
                        '_'
                    } else {
                        c
                    }
                } else {
                    c
                }
            })
            .collect();
        s.call_on_id("content_text", |view: &mut TextView| {
            view.set_content(masked_content)
        })
        .unwrap();
    }
}
