//! GTK 4 shell. All playback and mode rules live in `mineral-gui-core`.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, CssProvider, DrawingArea, Entry, Label,
    ListBox, ListBoxRow, Orientation, PolicyType, Scale, ScrolledWindow, Separator, glib,
};
use mineral_gui_core::{BridgeBackend, Session, Snapshot};
use serde_json::{Value, json};

struct Shell {
    session: Session<BridgeBackend>,
    generation: u64,
    was_mini: bool,
    rows_sig: String,
    lyrics_sig: String,
    tools_sig: String,
    overlay_sig: String,
    top: GtkBox,
    mode: Button,
    back: Button,
    daemon: Label,
    search: Entry,
    filter: Entry,
    title: Label,
    crumbs: Label,
    subtitle: Label,
    time_label: Label,
    empty: Label,
    notice: Label,
    play: Button,
    position: Scale,
    volume: Scale,
    mode_label: Label,
    rows: ListBox,
    lyrics: Label,
    lyric_rows: ListBox,
    viz: DrawingArea,
    tools: GtkBox,
    peaks: Rc<RefCell<Vec<f32>>>,
    scope: Rc<RefCell<Vec<f32>>>,
    waterfall: Rc<RefCell<Vec<Vec<f32>>>>,
    style: Rc<RefCell<String>>,
    left: GtkBox,
    right: GtkBox,
    center: GtkBox,
    body: GtkBox,
    player: GtkBox,
    overlay: GtkBox,
    overlay_search: Entry,
    overlay_list: ListBox,
    overlay_title: Label,
    close_box: GtkBox,
    status: Label,
    window: ApplicationWindow,
}

fn format_ms(ms: u64) -> String {
    let total_sec = ms / 1000;
    format!("{:02}:{:02}", total_sec / 60, total_sec % 60)
}

fn main() {
    let app = Application::builder().application_id("dev.mineral.gui.gtk").build();
    app.connect_activate(build);
    app.run_with_args(&["mineral-gui-gtk"]);
}

fn build(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title("Mineral").default_width(1380).default_height(860).build();
    let css = CssProvider::new();
    css.load_from_data(
        "window { background:#FFFFFF; color:#2B2625; }\
         box, list, row, label, headerbar { background:transparent; color:#2B2625; }\
         entry { background:#F2AE98; color:#FFFFFF; border-radius:18px; border:none; padding:6px 14px; }\
         button { background:#DFB5A6; color:#FFFFFF; border:none; border-radius:14px; padding:5px 14px; font-weight:600; }\
         button:hover { background:#E07A5F; color:#FFFFFF; }\
         button.play { background:#EA957A; color:#FFFFFF; border:none; border-radius:26px; font-weight:700; padding:8px 20px; }\
         scale trough { min-height:6px; min-width:6px; background:#C8B8AF; border-radius:3px; }\
         scale highlight { min-height:6px; min-width:6px; background:#E07A5F; border-radius:3px; }\
         scale slider { min-width:8px; min-height:18px; margin:0px; background:#E07A5F; border-radius:4px; }\
         scrollbar slider { min-width:5px; min-height:5px; margin:2px; background:#DFDAD6; border-radius:3px; }\
         .sidebar { background:#EA957A; color:#FFFFFF; border:none; padding:14px; }\
         .sidebar label, .sidebar button { color:#FFFFFF; background:transparent; }\
         .player { background:#DFCEC4; border:none; border-radius:0px; padding:16px; }\
         .card { background:#FFFFFF; border:1px solid #EFECE9; border-radius:14px; padding:16px; }\
         .heading { font-weight:700; font-size:18px; color:#2B2625; }\
         .muted, .error { color:#9E9692; }\
         row:selected { background:#FAF7F5; color:#2B2625; border-radius:8px; }\
         .touch button, .touch row { min-height:44px; }",
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("display"),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let root = GtkBox::new(Orientation::Vertical, 6);
    root.set_margin_start(8);
    root.set_margin_end(8);
    root.set_margin_top(6);
    root.set_margin_bottom(6);

    let top = GtkBox::new(Orientation::Horizontal, 6);
    let mode = Button::with_label("模式");
    let back = Button::with_label("← 返回");
    back.set_visible(false);
    let search = Entry::builder().placeholder_text("搜索歌曲、艺人、专辑、歌单（回车搜索）").hexpand(true).build();
    let filter = Entry::builder().placeholder_text("过滤当前列表 /").build();
    let cmd_btn = Button::with_label("命令 (Ctrl+K)");
    cmd_btn.set_widget_name("top-palette");
    let daemon = Label::new(Some("Daemon"));
    top.append(&mode);
    top.append(&back);
    top.append(&search);
    top.append(&filter);
    top.append(&cmd_btn);
    top.append(&daemon);

    let tools_scroll = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Never)
        .min_content_height(38)
        .build();
    let tools = GtkBox::new(Orientation::Horizontal, 4);
    tools_scroll.set_child(Some(&tools));

    let body = GtkBox::new(Orientation::Horizontal, 8);
    body.set_vexpand(true);
    let left = GtkBox::new(Orientation::Vertical, 4);
    left.add_css_class("sidebar");
    left.set_size_request(170, -1);
    for (id, title) in [("library", "曲库"), ("playlists", "歌单"), ("loved", "收藏"), ("downloads", "下载"), ("settings", "设置")] {
        let button = Button::with_label(title);
        button.set_widget_name(id);
        left.append(&button);
    }
    let center = GtkBox::new(Orientation::Vertical, 4);
    center.set_hexpand(true);
    center.set_vexpand(true);
    let header_row = GtkBox::new(Orientation::Horizontal, 8);
    let title = Label::new(Some("曲库"));
    title.add_css_class("heading");
    title.set_xalign(0.0);
    let crumbs = Label::new(None);
    crumbs.add_css_class("muted");
    crumbs.set_xalign(0.0);
    crumbs.set_hexpand(true);
    header_row.append(&title);
    header_row.append(&crumbs);
    let empty = Label::new(None);
    empty.add_css_class("muted");
    empty.set_xalign(0.0);
    let rows = ListBox::new();
    rows.set_selection_mode(gtk4::SelectionMode::Single);
    rows.set_activate_on_single_click(true);
    let scroll = ScrolledWindow::builder().child(&rows).hexpand(true).vexpand(true).hscrollbar_policy(PolicyType::Never).build();
    center.append(&header_row);
    center.append(&empty);
    center.append(&scroll);

    let right = GtkBox::new(Orientation::Vertical, 6);
    right.set_size_request(310, -1);
    let lyrics = Label::new(Some("歌词"));
    lyrics.add_css_class("muted");
    lyrics.set_wrap(true);
    lyrics.set_xalign(0.0);
    let lyric_rows = ListBox::new();
    lyric_rows.set_selection_mode(gtk4::SelectionMode::Single);
    lyric_rows.set_activate_on_single_click(true);
    let lyric_scroll = ScrolledWindow::builder()
        .child(&lyric_rows)
        .vexpand(true)
        .hscrollbar_policy(PolicyType::Never)
        .build();
    let viz = DrawingArea::new();
    viz.set_content_height(110);
    right.append(&Label::new(Some("正在播放与歌词")));
    right.append(&lyrics);
    right.append(&lyric_scroll);
    right.append(&viz);

    body.append(&left);
    body.append(&Separator::new(Orientation::Vertical));
    body.append(&center);
    body.append(&Separator::new(Orientation::Vertical));
    body.append(&right);

    let player = GtkBox::new(Orientation::Horizontal, 6);
    player.add_css_class("player");
    let play = Button::with_label("播放");
    play.add_css_class("play");
    let subtitle = Label::new(Some("没有正在播放"));
    subtitle.set_hexpand(true);
    subtitle.set_xalign(0.0);
    let time_label = Label::new(Some("00:00 / 00:00"));
    time_label.add_css_class("muted");
    let position = Scale::with_range(Orientation::Horizontal, 0.0, 1000.0, 1000.0);
    position.set_hexpand(true);
    position.set_draw_value(false);
    let volume = Scale::with_range(Orientation::Horizontal, 0.0, 100.0, 1.0);
    volume.set_value(80.0);
    volume.set_size_request(100, -1);
    volume.set_draw_value(false);
    let mode_label = Label::new(Some("顺序"));
    for (label, action) in [
        ("⏮", "prev"),
        ("⏭", "next"),
        ("队列", "queue_drawer"),
        ("词轨", "lyric_track"),
        ("循环", "cycle_mode"),
        ("沉浸", "scheme"),
        ("收藏", "love"),
        ("入队", "enqueue"),
        ("展开", "back"),
    ] {
        let button = Button::with_label(label);
        button.set_widget_name(action);
        player.append(&button);
    }
    player.append(&play);
    player.append(&subtitle);
    player.append(&time_label);
    player.append(&position);
    player.append(&volume);
    player.append(&mode_label);

    let notice = Label::new(None);
    notice.set_xalign(0.0);
    let status = Label::new(None);
    status.add_css_class("muted");
    status.set_xalign(0.0);
    let overlay = GtkBox::new(Orientation::Vertical, 8);
    overlay.add_css_class("card");
    overlay.set_halign(gtk4::Align::Center);
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_size_request(540, 420);
    overlay.set_visible(false);
    let overlay_top = GtkBox::new(Orientation::Horizontal, 8);
    let overlay_title = Label::new(Some("模式中心"));
    overlay_title.add_css_class("heading");
    overlay_title.set_xalign(0.0);
    overlay_title.set_hexpand(true);
    let overlay_close = Button::with_label("关闭 (Esc)");
    overlay_close.set_widget_name("overlay-close");
    overlay_top.append(&overlay_title);
    overlay_top.append(&overlay_close);
    let overlay_search = Entry::builder().placeholder_text("输入关键词筛选模式或命令…").build();
    let overlay_list = ListBox::new();
    overlay_list.set_activate_on_single_click(true);
    let overlay_scroll = ScrolledWindow::builder()
        .child(&overlay_list)
        .vexpand(true)
        .hscrollbar_policy(PolicyType::Never)
        .build();
    overlay.append(&overlay_top);
    overlay.append(&overlay_search);
    overlay.append(&overlay_scroll);

    let close_box = GtkBox::new(Orientation::Horizontal, 8);
    close_box.set_visible(false);
    close_box.append(&Label::new(Some("关闭窗口时：")));
    let close_ui = Button::with_label("仅关闭界面");
    let close_stop = Button::with_label("停止播放");
    let close_cancel = Button::with_label("取消");
    close_ui.set_widget_name("close-ui");
    close_stop.set_widget_name("close-stop");
    close_cancel.set_widget_name("close-cancel");
    close_box.append(&close_ui);
    close_box.append(&close_stop);
    close_box.append(&close_cancel);

    root.append(&top);
    root.append(&tools_scroll);
    root.append(&notice);
    root.append(&body);
    root.append(&close_box);
    root.append(&player);
    root.append(&status);
    let host = gtk4::Overlay::new();
    host.set_child(Some(&root));
    host.add_overlay(&overlay);
    window.set_child(Some(&host));

    let peaks = Rc::new(RefCell::new(Vec::<f32>::new()));
    let scope = Rc::new(RefCell::new(Vec::<f32>::new()));
    let style = Rc::new(RefCell::new("bars".to_owned()));
    let peaks_draw = Rc::clone(&peaks);
    let scope_draw = Rc::clone(&scope);
    let style_draw = Rc::clone(&style);
    let waterfall = Rc::new(RefCell::new(Vec::<Vec<f32>>::new()));
    let waterfall_draw = Rc::clone(&waterfall);
    viz.set_draw_func(move |_area, cr, width, height| {
        cr.set_source_rgb(0.12, 0.13, 0.15);
        cr.paint().ok();
        let style = style_draw.borrow().clone();
        let peaks = peaks_draw.borrow().clone();
        if style == "waterfall" {
            let history = waterfall_draw.borrow().clone();
            let rows = history.len().max(1) as f64;
            for (row_index, line) in history.iter().enumerate() {
                let count = line.len().max(1) as f64;
                for (index, peak) in line.iter().enumerate() {
                    let amp = f64::from(peak.clamp(0.0, 1.0));
                    cr.set_source_rgb(0.15 + 0.75 * amp, 0.22, 0.12);
                    cr.rectangle(index as f64 * width as f64 / count, row_index as f64 * height as f64 / rows, width as f64 / count, height as f64 / rows);
                    cr.fill().ok();
                }
            }
            return;
        }
        cr.set_source_rgb(0.82, 0.54, 0.30);
        if style == "scope" {
            let scope = scope_draw.borrow().clone();
            cr.move_to(0.0, height as f64 / 2.0);
            for (index, sample) in scope.iter().enumerate() {
                let x = index as f64 / scope.len().max(1) as f64 * width as f64;
                let y = height as f64 / 2.0 - f64::from(*sample) * height as f64 / 2.0;
                cr.line_to(x, y);
            }
            cr.stroke().ok();
        } else {
            let count = peaks.len().max(1) as f64;
            for (index, peak) in peaks.iter().enumerate() {
                let bar_w = width as f64 / count;
                let bar_h = f64::from(peak.clamp(0.0, 1.0)) * height as f64;
                let x = index as f64 * bar_w;
                if style == "terrain" {
                    cr.line_to(x, height as f64 - bar_h);
                } else {
                    cr.rectangle(x + 1.0, height as f64 - bar_h, bar_w - 2.0, bar_h);
                }
            }
            if style == "terrain" { cr.stroke().ok(); } else { cr.fill().ok(); }
        }
    });

    let shell = Rc::new(RefCell::new(Shell {
        session: Session::open(BridgeBackend::launch(), mineral_gui_core::session_path()),
        generation: 0,
        was_mini: false,
        rows_sig: String::new(),
        lyrics_sig: String::new(),
        tools_sig: String::new(),
        overlay_sig: String::new(),
        top,
        mode,
        back,
        daemon,
        search,
        filter,
        title,
        crumbs,
        subtitle,
        time_label,
        empty,
        notice,
        play,
        position,
        volume,
        mode_label,
        rows,
        lyrics,
        lyric_rows,
        viz,
        tools,
        peaks,
        scope,
        waterfall,
        style,
        left,
        right,
        center,
        body,
        player,
        overlay,
        overlay_search,
        overlay_list,
        overlay_title,
        close_box,
        status,
        window: window.clone(),
    }));

    let applying = Rc::new(Cell::new(false));
    wire(&shell, &applying);
    refresh(&shell, &applying);
    let tick = Rc::clone(&shell);
    let tick_apply = Rc::clone(&applying);
    let autotest = mineral_gui_core::is_autotest();
    let step = Rc::new(Cell::new(0usize));
    glib::timeout_add_local(Duration::from_millis(60), move || {
        tick.borrow_mut().session.tick();
        if autotest {
            let next_step = step.get() + 1;
            step.set(next_step);
            if let Some(action) = mineral_gui_core::autotest_step(next_step) {
                tick.borrow_mut().session.dispatch_value(&action);
            }
        }
        let quit = tick.borrow().session.quit;
        refresh(&tick, &tick_apply);
        if quit {
            let win = tick.borrow().window.clone();
            tick.borrow_mut().session.close();
            win.close();
            return glib::ControlFlow::Break;
        }
        glib::ControlFlow::Continue
    });
    window.present();
}

fn send(shell: &Rc<RefCell<Shell>>, applying: &Rc<Cell<bool>>, value: Value) {
    if applying.get() {
        return;
    }
    shell.borrow_mut().session.dispatch_value(&value);
    refresh(shell, applying);
}

fn wire(shell: &Rc<RefCell<Shell>>, applying: &Rc<Cell<bool>>) {
    let ui = shell.borrow();
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.mode.connect_clicked(move |_| send(&shell, &applying, json!({"action":"mode_center"})));
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.back.connect_clicked(move |_| send(&shell, &applying, json!({"action":"back"})));
    }
    if let Some(cmd_btn) = ui.top.first_child().and_then(|w| find_named(&w, "top-palette")) {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        cmd_btn.connect_clicked(move |_| send(&shell, &applying, json!({"action":"palette"})));
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.search.connect_activate(move |entry| send(&shell, &applying, json!({"action":"search","query": entry.text().as_str()})));
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.filter.connect_changed(move |entry| send(&shell, &applying, json!({"action":"list_filter","query": entry.text().as_str()})));
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.overlay_search.connect_changed(move |entry| {
            let text = entry.text().to_string();
            let is_mode = shell.borrow().session.snapshot().mode_center_open;
            if is_mode {
                send(&shell, &applying, json!({"action":"mode_query","query": text}));
            } else {
                send(&shell, &applying, json!({"action":"palette_query","query": text}));
            }
        });
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.overlay_search.connect_activate(move |_| {
            let snap = shell.borrow().session.snapshot();
            if snap.palette_open
                && let Some(first) = snap.commands.first()
            {
                send(&shell, &applying, json!({"action":"run_command","command": first.id}));
            }
        });
    }
    if let Some(close_btn) = ui.overlay.first_child().and_then(|w| w.first_child()).and_then(|w| find_named(&w, "overlay-close")) {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        close_btn.connect_clicked(move |_| send(&shell, &applying, json!({"action":"back"})));
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.play.connect_clicked(move |_| send(&shell, &applying, json!({"action":"toggle_play"})));
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.position.connect_change_value(move |_scale, _scroll, value| {
            send(&shell, &applying, json!({"action":"seek","ms": value as u64}));
            glib::Propagation::Stop
        });
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.volume.connect_value_changed(move |scale| send(&shell, &applying, json!({"action":"volume","pct": scale.value() as i64})));
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.rows.connect_row_selected(move |_list, row| {
            if let Some(row) = row {
                let key = row.widget_name().to_string();
                send(&shell, &applying, json!({"action":"select","key": key}));
            }
        });
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.rows.connect_row_activated(move |_list, row| {
            let key = row.widget_name().to_string();
            let snap = shell.borrow().session.snapshot();
            if let Some(action) = settings_action(&key, &snap) {
                send(&shell, &applying, action);
            } else {
                send(&shell, &applying, json!({"action":"activate","key": key}));
            }
        });
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.lyric_rows.connect_row_activated(move |_list, row| {
            let ms = row.widget_name().parse::<u64>().unwrap_or(0);
            send(&shell, &applying, json!({"action":"lyric_seek","ms": ms}));
        });
    }
    let mut child = ui.left.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        if let Some(button) = widget.downcast_ref::<Button>() {
            let shell = Rc::clone(shell);
            let name = button.widget_name();
            let applying = Rc::clone(applying);
            button.connect_clicked(move |_| send(&shell, &applying, json!({"action":"nav","nav": name.as_str()})));
        }
    }
    let mut child = ui.player.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        let Some(button) = widget.downcast_ref::<Button>() else { continue };
        let name = button.widget_name().to_string();
        if name.is_empty() { continue; }
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        button.connect_clicked(move |_| {
            let value = if name == "scheme" {
                json!({"action":"scheme","scheme":"immersive"})
            } else if matches!(name.as_str(), "queue_drawer" | "lyric_track" | "cycle_mode" | "back") {
                json!({"action": name})
            } else {
                json!({"action": name, "key": ""})
            };
            send(&shell, &applying, value);
        });
    }
    for (name, choice) in [("close-ui", "ui"), ("close-stop", "stop"), ("close-cancel", "cancel")] {
        let shell = Rc::clone(shell);
        if let Some(button) = ui.close_box.first_child().and_then(|w| find_named(&w, name)) {
            let choice = choice.to_owned();
            let applying = Rc::clone(applying);
            button.connect_clicked(move |_| send(&shell, &applying, json!({"action":"confirm_close","choice": choice})));
        }
    }
    {
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        ui.overlay_list.connect_row_activated(move |_list, row| {
            let id = row.widget_name().to_string();
            if id.starts_with("scheme:") {
                send(&shell, &applying, json!({"action":"scheme","scheme": id.trim_start_matches("scheme:")}));
            } else if id.starts_with("combo:") {
                send(&shell, &applying, json!({"action":"apply_combo","name": id.trim_start_matches("combo:")}));
            } else {
                send(&shell, &applying, json!({"action":"run_command","command": id}));
            }
        });
    }
    let key_shell = Rc::clone(shell);
    let key_apply = Rc::clone(applying);
    let controller = gtk4::EventControllerKey::new();
    controller.connect_key_pressed(move |_ctrl, key, _code, mods| {
        let typing = key_shell.borrow().search.has_focus()
            || key_shell.borrow().filter.has_focus()
            || key_shell.borrow().overlay_search.has_focus();
        key_shell.borrow_mut().session.dispatch_value(&json!({"action":"set_typing","on": typing}));
        if handle_key(&key_shell, &key_apply, key, mods, typing) {
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    ui.window.add_controller(controller);
    {
        let shell = Rc::clone(shell);
        let close_apply = Rc::clone(applying);
        ui.window.connect_close_request(move |window| {
            let already_quit = shell.borrow().session.quit;
            if !already_quit {
                shell.borrow_mut().session.dispatch_value(&json!({"action":"request_close"}));
                let snap = shell.borrow().session.snapshot();
                refresh(&shell, &close_apply);
                if snap.close_prompt || !snap.quit {
                    return glib::Propagation::Stop;
                }
            }
            shell.borrow_mut().session.close();
            window.destroy();
            glib::Propagation::Proceed
        });
    }
    drop(ui);
}

fn find_named(widget: &gtk4::Widget, name: &str) -> Option<Button> {
    let mut current = Some(widget.clone());
    while let Some(widget) = current {
        if widget.widget_name() == name {
            return widget.downcast::<Button>().ok();
        }
        current = widget.next_sibling();
    }
    None
}

fn handle_key(shell: &Rc<RefCell<Shell>>, applying: &Rc<Cell<bool>>, key: Key, mods: gtk4::gdk::ModifierType, typing: bool) -> bool {
    let ctrl = mods.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
    let shift = mods.contains(gtk4::gdk::ModifierType::SHIFT_MASK);
    let action = if ctrl && shift && (key == Key::m || key == Key::M) {
        Some(json!({"action":"mode_center"}))
    } else if ctrl && shift && key == Key::BackSpace {
        Some(json!({"action":"return_mode"}))
    } else if ctrl && !shift && (key == Key::k || key == Key::K) {
        Some(json!({"action":"palette"}))
    } else if key == Key::Escape {
        Some(json!({"action":"back"}))
    } else if typing {
        None
    } else if key == Key::space {
        Some(json!({"action":"toggle_play"}))
    } else if key == Key::Return && shift {
        Some(json!({"action":"enqueue","key":""}))
    } else if key == Key::Return {
        Some(json!({"action":"activate","key":""}))
    } else if key == Key::j && !ctrl {
        Some(json!({"action":"select_delta","delta": 1}))
    } else if key == Key::k && !ctrl {
        Some(json!({"action":"select_delta","delta": -1}))
    } else if key == Key::n || key == Key::N {
        Some(json!({"action":"next"}))
    } else if key == Key::p || key == Key::P {
        Some(json!({"action":"prev"}))
    } else if key == Key::l || key == Key::L {
        Some(json!({"action":"love","key":""}))
    } else if key == Key::d || key == Key::D {
        Some(json!({"action":"download","key":""}))
    } else if key == Key::t || key == Key::T {
        Some(json!({"action":"lyric_track"}))
    } else if key == Key::z || key == Key::Z {
        Some(json!({"action":"scheme","scheme":"immersive"}))
    } else if key == Key::Tab {
        Some(json!({"action":"queue_drawer"}))
    } else if key == Key::c || key == Key::C {
        Some(json!({"action":"reveal"}))
    } else if key == Key::m || key == Key::M {
        Some(json!({"action":"cycle_mode"}))
    } else if key == Key::slash {
        shell.borrow().filter.grab_focus();
        None
    } else if key == Key::question {
        Some(json!({"action":"help","open":true}))
    } else if ctrl && (key == Key::j || key == Key::J) {
        Some(json!({"action":"queue_move_selected","to":"down"}))
    } else if shift && key == Key::Up {
        Some(json!({"action":"queue_move_selected","to":"up"}))
    } else if shift && key == Key::Down {
        Some(json!({"action":"queue_move_selected","to":"down"}))
    } else if key == Key::Delete {
        Some(json!({"action":"queue_remove_selected"}))
    } else {
        None
    };
    if let Some(action) = action {
        send(shell, applying, action);
        true
    } else {
        false
    }
}

fn refresh(shell: &Rc<RefCell<Shell>>, applying: &Rc<Cell<bool>>) {
    applying.set(true);
    let mut need_tools = None;
    {
        let mut ui = shell.borrow_mut();
        let snap = ui.session.snapshot();
        let dur = snap.duration_ms.unwrap_or(0);
        ui.time_label.set_text(&format!("{} / {}", format_ms(snap.position_ms), format_ms(dur)));
        if snap.generation == ui.generation {
            ui.play.set_label(if snap.playing { "暂停" } else { "播放" });
            ui.position.set_value(snap.position_ms as f64);
            if let Some(duration) = snap.duration_ms {
                ui.position.set_range(0.0, duration.max(1) as f64);
            }
        } else {
            ui.generation = snap.generation;
            apply_snapshot(&mut ui, &snap);
            need_tools = Some(snap);
        }
    }
    if let Some(snap) = need_tools {
        fill_tools(shell, applying, &snap);
    }
    applying.set(false);
}

fn apply_snapshot(ui: &mut Shell, snap: &Snapshot) {
    ui.mode.set_label(&format!("模式 · {}{}", snap.scheme_label, if snap.locked { " 🔒" } else { "" }));
    ui.daemon.set_text(if snap.daemon_connected { "● 已连接" } else { "○ 未连接" });
    let can_back = snap.crumbs.len() > 1 || (snap.content_kind == "search" && snap.workspace != "spotlight");
    ui.back.set_visible(can_back);
    ui.crumbs.set_text(&if snap.crumbs.len() > 1 { snap.crumbs.join("  ›  ") } else { String::new() });

    let mut heading = snap.content_title.clone();
    if snap.content_kind == "grid" {
        heading.push_str(" · 封面网格");
    }
    if snap.workspace == "modular" {
        heading.push_str(&format!(" · {}", snap.modular_preset));
    }
    ui.title.set_text(&heading);
    let next_hint = if snap.next_title.is_empty() {
        String::new()
    } else {
        format!("  (下一首：{})", snap.next_title)
    };
    ui.subtitle.set_text(&format!("{}  {}{}", snap.player_title, snap.player_subtitle, next_hint));
    ui.empty.set_text(&snap.empty);
    ui.empty.set_visible(!snap.empty.is_empty());
    ui.play.set_label(if snap.playing { "暂停" } else { "播放" });
    ui.mode_label.set_text(&snap.play_mode);
    ui.position.set_range(0.0, snap.duration_ms.unwrap_or(1).max(1) as f64);
    ui.position.set_value(snap.position_ms as f64);
    if (ui.volume.value() - f64::from(snap.volume)).abs() > 0.5 {
        ui.volume.set_value(f64::from(snap.volume));
    }
    let notice_text = snap.notices.iter().map(|notice| format!("[{}] {}", notice.level, notice.text)).collect::<Vec<_>>().join("  ·  ");
    ui.notice.set_text(&notice_text);
    ui.notice.set_visible(!notice_text.is_empty());
    ui.status.set_text(&format!(
        "{}{}{}",
        if snap.loading { "正在加载… · " } else { "" },
        if snap.play_error.is_empty() { &snap.daemon_message } else { &snap.play_error },
        if snap.selected_title.is_empty() { String::new() } else { format!(" · 选中：{}", snap.selected_title) }
    ));
    ui.lyrics.set_text(if snap.lyrics.is_empty() { "暂无歌词" } else { "lyrics · synced" });
    let next_lyrics_sig: String = snap.lyrics.iter().map(|l| format!("{}:{}:{}:{}\n", l.t.unwrap_or(0), l.active, l.text, l.sub_text)).collect();
    if ui.lyrics_sig != next_lyrics_sig {
        ui.lyrics_sig = next_lyrics_sig;
        clear_list(&ui.lyric_rows);
        for line in &snap.lyrics {
            let mark = if line.active { "▶ " } else { "   " };
            let text = if line.sub_text.is_empty() {
                format!("{mark}{}", line.text)
            } else {
                format!("{mark}{}\n     {}", line.text, line.sub_text)
            };
            add_row(&ui.lyric_rows, &line.t.unwrap_or(0).to_string(), &text, line.active);
        }
    }
    *ui.peaks.borrow_mut() = snap.peaks.clone();
    *ui.scope.borrow_mut() = snap.scope.clone();
    *ui.waterfall.borrow_mut() = snap.waterfall.clone();
    *ui.style.borrow_mut() = snap.viz_style.clone();
    ui.viz.queue_draw();
    ui.window.set_css_classes(if snap.touch { &["touch"] } else { &[] });

    let mini = snap.mini;
    let immersive = snap.immersive;
    if mini && !ui.was_mini {
        ui.was_mini = true;
        ui.window.set_default_size(560, 140);
    } else if !mini && ui.was_mini {
        ui.was_mini = false;
        ui.window.set_default_size(1220, 780);
    }
    ui.top.set_visible(!mini);
    ui.tools.set_visible(!mini && !immersive);
    ui.left.set_visible(!mini && !immersive && snap.workspace != "spotlight" && snap.workspace != "single" && (snap.workspace != "modular" || snap.modules.iter().any(|m| m == "navigation")));
    ui.right.set_visible(!mini && (immersive || matches!(snap.workspace.as_str(), "classic" | "queue" | "covers") || (snap.workspace == "single" && matches!(snap.single_drawer.as_str(), "lyrics" | "details")) || (snap.workspace == "modular" && snap.modules.iter().any(|m| matches!(m.as_str(), "lyrics" | "visualizer" | "details")))));
    ui.center.set_visible(!mini && !immersive);
    ui.body.set_visible(!mini);
    ui.player.set_visible(true);
    ui.close_box.set_visible(snap.close_prompt);

    let show_overlay = snap.mode_center_open || snap.palette_open || snap.help_open;
    ui.overlay.set_visible(show_overlay);
    ui.overlay_search.set_visible(snap.mode_center_open || snap.palette_open);
    if show_overlay {
        let next_overlay_sig = format!(
            "{}:{}:{}:{}:{}:{}",
            snap.mode_center_open,
            snap.palette_open,
            snap.help_open,
            snap.scheme_id,
            snap.schemes.len(),
            snap.commands.iter().map(|c| c.id.as_str()).collect::<Vec<_>>().join(",")
        );
        if ui.overlay_sig != next_overlay_sig {
            ui.overlay_sig = next_overlay_sig;
            clear_list(&ui.overlay_list);
            if snap.mode_center_open {
                ui.overlay_title.set_text("模式中心 · 十种方案");
                for scheme in &snap.schemes {
                    let prefix = if scheme.current { "● 当前 · " } else { "" };
                    add_row(&ui.overlay_list, &format!("scheme:{}", scheme.id), &format!("{prefix}{} — {}", scheme.title, scheme.purpose), scheme.current);
                }
                for name in &snap.custom_modes {
                    add_row(&ui.overlay_list, &format!("combo:{name}"), &format!("★ 组合 · {name}"), false);
                }
            } else if snap.palette_open {
                ui.overlay_title.set_text("命令与全局跳转面板");
                for command in &snap.commands {
                    add_row(&ui.overlay_list, &command.id, &command.title, false);
                }
            } else {
                ui.overlay_title.set_text("快捷键指南");
                for line in [
                    "Space：播放 / 暂停",
                    "Enter / 双击：播放歌曲 或 打开详情",
                    "Shift + Enter：加入播放队列",
                    "j / k：上下选择列表项",
                    "N / P：下一首 / 上一首",
                    "/：聚焦当前列表过滤器",
                    "Tab：打开 / 关闭队列抽屉",
                    "T / M：切换歌词轨道 / 循环模式",
                    "Z / Esc：沉浸模式 / 返回上一层",
                    "Ctrl + K：命令与全局跳转面板",
                    "Ctrl + Shift + M：模式中心（10 种交互模式）",
                ] {
                    add_row(&ui.overlay_list, "help", line, false);
                }
            }
        }
    }

    let queue_showing = snap.workspace == "queue" || snap.queue_open || (snap.workspace == "single" && snap.single_drawer == "queue");
    let source_rows: Vec<_> = if queue_showing && snap.workspace != "queue" {
        snap.queue.iter().collect()
    } else {
        snap.rows.iter().collect()
    };
    let next_rows_sig: String = source_rows
        .iter()
        .map(|r| format!("{}:{}:{}:{}:{}:{}\n", r.key, r.title, r.subtitle, r.album, r.loved, r.current))
        .collect();
    if ui.rows_sig != next_rows_sig {
        ui.rows_sig = next_rows_sig;
        clear_list(&ui.rows);
        for row in source_rows {
            let mark = if row.current { "▶ " } else if row.loved { "♥ " } else { "" };
            let badge = match row.kind.as_str() {
                "playlist" => "[歌单] ",
                "album" => "[专辑] ",
                "artist" => "[艺人] ",
                "source" => "[音源] ",
                _ => "",
            };
            let album_str = if row.album.is_empty() { String::new() } else { format!(" · {}", row.album) };
            let dur = row.duration_ms.map(|d| format!(" · {}", format_ms(d))).unwrap_or_default();
            let text = format!("{mark}{badge}{}   {}{album_str}   {}{dur}", row.title, row.subtitle, row.source);
            add_row(&ui.rows, &row.key, &text, row.key == snap.selected_key);
        }
    }
    ui.window.set_title(Some(&format!("Mineral · {}", snap.scheme_label)));
}

fn clear_list(list: &ListBox) {
    while let Some(row) = list.row_at_index(0) {
        list.remove(&row);
    }
}

fn settings_action(key: &str, snap: &Snapshot) -> Option<Value> {
    Some(match key {
        "settings:close" => json!({"action":"close_policy","policy": match snap.close_policy.as_str() { "ask" => "ui", "ui" => "stop", _ => "ask" }}),
        "settings:adapt" => json!({"action":"auto_adapt","on": !snap.auto_adapt}),
        "settings:lock" => json!({"action":"lock","on": !snap.locked}),
        "settings:motion" => json!({"action":"reduce_motion","on": !snap.reduce_motion}),
        "settings:reset-layout" => json!({"action":"reset_layout"}),
        "settings:reset-modes" => json!({"action":"reset_modes"}),
        "settings:daemon" => json!({"action": if snap.daemon_connected { "connect" } else { "launch_daemon" }}),
        _ => return None,
    })
}

fn fill_tools(shell: &Rc<RefCell<Shell>>, applying: &Rc<Cell<bool>>, snap: &Snapshot) {
    let next_tools_sig = format!(
        "{}:{}:{}:{}:{}:{}",
        snap.workspace,
        snap.content_kind,
        snap.queue_open,
        snap.single_drawer,
        snap.can_undo_auto,
        snap.sources.iter().map(|s| s.id.as_str()).collect::<Vec<_>>().join(",")
    );
    if shell.borrow().tools_sig == next_tools_sig {
        return;
    }
    shell.borrow_mut().tools_sig = next_tools_sig;
    let tools = shell.borrow().tools.clone();
    while let Some(child) = tools.first_child() {
        tools.remove(&child);
    }
    let mut buttons: Vec<(String, Value)> = vec![
        ("播放全部".into(), json!({"action":"play_container","key":""})),
        ("下一首播".into(), json!({"action":"play_next","key":""})),
        ("入队".into(), json!({"action":"enqueue","key":""})),
        ("下载".into(), json!({"action":"download","key":""})),
        ("频谱".into(), json!({"action":"viz","style": next_viz(&snap.viz_style)})),
        ("保存组合".into(), json!({"action":"save_combo","name":""})),
    ];
    if snap.can_undo_auto {
        buttons.push(("撤销自动切换".into(), json!({"action":"undo_auto"})));
    }
    buttons.push(("全部音源".into(), json!({"action":"source_filter","source":""})));
    for source in &snap.sources {
        let label = if source.label.is_empty() { source.id.clone() } else { source.label.clone() };
        buttons.push((format!("音源:{label}"), json!({"action":"source_filter","source": source.id})));
    }
    if snap.content_kind == "search" {
        for (tid, tlabel) in [("all", "搜:全部"), ("song", "搜:单曲"), ("artist", "搜:艺人"), ("album", "搜:专辑"), ("playlist", "搜:歌单")] {
            buttons.push((tlabel.into(), json!({"action":"search_tab","tab": tid})));
        }
    }
    if snap.workspace == "covers" || snap.content_kind == "grid" {
        buttons.push(("封面网格".into(), json!({"action":"cover_mode","mode":"grid"})));
        buttons.push(("紧凑列表".into(), json!({"action":"cover_mode","mode":"list"})));
    }
    if snap.workspace == "queue" || snap.queue_open || (snap.workspace == "single" && snap.single_drawer == "queue") {
        buttons.push(("队列上移".into(), json!({"action":"queue_move_selected","to":"up"})));
        buttons.push(("队列下移".into(), json!({"action":"queue_move_selected","to":"down"})));
        buttons.push(("移到队首".into(), json!({"action":"queue_move_selected","to":"top"})));
        buttons.push(("移除选中".into(), json!({"action":"queue_remove_selected"})));
        buttons.push(("队列去重".into(), json!({"action":"queue_dedupe"})));
        buttons.push(("清空队列".into(), json!({"action":"queue_clear"})));
        buttons.push(("定位当前".into(), json!({"action":"reveal"})));
    }
    if snap.workspace == "modular" {
        for (preset, title) in [("browse", "预设:浏览"), ("queue", "预设:队列"), ("lyrics", "预设:歌词"), ("viz", "预设:频谱")] {
            buttons.push((title.into(), json!({"action":"modular_preset","preset": preset})));
        }
        for (mid, mtitle) in [("navigation", "模块:导航"), ("library", "模块:主区"), ("lyrics", "模块:歌词"), ("visualizer", "模块:频谱")] {
            buttons.push((mtitle.into(), json!({"action":"toggle_module","module": mid})));
        }
        buttons.push(("重置布局".into(), json!({"action":"reset_layout"})));
    }
    if snap.workspace == "single" {
        for (did, dtitle) in [("none", "抽屉:收起"), ("queue", "抽屉:队列"), ("lyrics", "抽屉:歌词"), ("details", "抽屉:详情")] {
            buttons.push((dtitle.into(), json!({"action":"single_drawer","drawer": did})));
        }
    }
    for (text, action) in buttons {
        let button = Button::with_label(&text);
        let shell = Rc::clone(shell);
        let applying = Rc::clone(applying);
        button.connect_clicked(move |_| {
            send(&shell, &applying, action.clone());
        });
        tools.append(&button);
    }
}

fn next_viz(style: &str) -> &'static str {
    match style {
        "bars" => "scope",
        "scope" => "waterfall",
        "waterfall" => "terrain",
        _ => "bars",
    }
}

fn add_row(list: &ListBox, key: &str, text: &str, current: bool) {
    let row = ListBoxRow::new();
    row.set_widget_name(key);
    let label = Label::new(Some(text));
    label.set_xalign(0.0);
    label.set_margin_top(6);
    label.set_margin_bottom(6);
    label.set_margin_start(8);
    if current {
        label.add_css_class("heading");
    }
    row.set_child(Some(&label));
    list.append(&row);
}
