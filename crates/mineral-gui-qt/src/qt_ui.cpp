#include <QApplication>
#include <QButtonGroup>
#include <QCloseEvent>
#include <QCoreApplication>
#include <QDateTime>
#include <QDialog>
#include <QDir>
#include <QEasingCurve>
#include <QFileDialog>
#include <QFileInfo>
#include <QFrame>
#include <QHBoxLayout>
#include <QHash>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QKeyEvent>
#include <QLabel>
#include <QLineEdit>
#include <QListWidget>
#include <QMainWindow>
#include <QMenu>
#include <QPainter>
#include <QPlainTextEdit>
#include <QProcess>
#include <QPushButton>
#include <QResizeEvent>
#include <QScreen>
#include <QScrollBar>
#include <QSettings>
#include <QSignalBlocker>
#include <QSlider>
#include <QSplitter>
#include <QStandardPaths>
#include <QStyle>
#include <QSystemTrayIcon>
#include <QTimer>
#include <QVBoxLayout>
#include <QVariantAnimation>
#include <QWidget>

#include <algorithm>
#include <cstring>
#include <deque>
#include <string>
#include <vector>

#include "qt_widgets.hpp"
#include "logo_data.hpp"

namespace {
std::deque<std::string> actions;
QApplication* app = nullptr;
QMainWindow* window = nullptr;
QWidget* header_bar = nullptr;
QPushButton* mode_button = nullptr;
QPushButton* undo_auto_btn = nullptr;
QLineEdit* search_box = nullptr;
QLineEdit* filter_box = nullptr;
QLineEdit* inline_filter_box = nullptr;
QLabel* daemon_label = nullptr;
QPushButton* daemon_btn = nullptr;
QPushButton* back_btn = nullptr;
QPushButton* content_back_btn = nullptr;
QLabel* crumbs_label = nullptr;
QLabel* heading = nullptr;
QLabel* stats_label = nullptr;
QLabel* summary_label = nullptr;
QLabel* empty_label = nullptr;
QWidget* col_header_bar = nullptr;
QWidget* notice_bar = nullptr;
QLabel* notice_label = nullptr;
QPushButton* notice_close_btn = nullptr;
QWidget* filter_chips_bar = nullptr;
QHBoxLayout* source_chips_layout = nullptr;
QWidget* search_tabs_bar = nullptr;
QButtonGroup* search_tab_group = nullptr;
QWidget* cover_tools_bar = nullptr;
QPushButton* cover_grid_btn = nullptr;
QPushButton* cover_list_btn = nullptr;
QWidget* modular_bar = nullptr;
QWidget* keyboard_bar = nullptr;
QWidget* touch_action_bar = nullptr;
QWidget* single_drawer_bar = nullptr;
mineral_ui::CoverArtWidget* cover_widget = nullptr;
QLabel* selected_source_badge = nullptr;
QLabel* sel_title_label = nullptr;
QLabel* sel_sub_label = nullptr;
QLabel* sel_meta_label = nullptr;
QPushButton* sel_love_btn = nullptr;
QLabel* now_title = nullptr;
QLabel* now_artist = nullptr;
QLabel* stream_badge = nullptr;
QLabel* pos_time_label = nullptr;
QLabel* dur_time_label = nullptr;
QLabel* next_up_label = nullptr;
QListWidget* rows = nullptr;
QListWidget* lib_browser_list = nullptr;
QListWidget* lyric_list = nullptr;
QListWidget* fullscreen_lyric_list = nullptr;
QListWidget* mode_list = nullptr;
QLineEdit* mode_search_box = nullptr;
QLineEdit* combo_name_box = nullptr;
QDialog* mode_dialog = nullptr;
QDialog* palette_dialog = nullptr;
QLineEdit* palette_search_box = nullptr;
QListWidget* palette_list = nullptr;
QDialog* help_dialog = nullptr;
QPushButton* play_button = nullptr;
QPushButton* mode_cycle_btn = nullptr;
QPushButton* lyric_cycle_btn = nullptr;
QPushButton* love_btn = nullptr;
QPushButton* dyn_color_btn = nullptr;
QPushButton* lyric_font_dec_btn = nullptr;
QPushButton* lyric_font_inc_btn = nullptr;
QPushButton* mini_restore_btn = nullptr;
QPushButton* sidebar_toggle_btn = nullptr;
QPushButton* search_icon_btn = nullptr;
QPushButton* help_rail_btn_ref = nullptr;
QFrame* search_bar_frame_ref = nullptr;
QSlider* position = nullptr;
QSlider* volume = nullptr;
mineral_ui::VolumeBarWidget* volume_bar = nullptr;
QWidget* sidebar = nullptr;
QVBoxLayout* rail_layout_ref = nullptr;
QHBoxLayout* side_top_row_ref = nullptr;
QHBoxLayout* side_footer_ref = nullptr;
QWidget* main_col_ref = nullptr;
QHBoxLayout* hero_h_layout_ref = nullptr;
QFrame* queue_left_panel = nullptr;
QFrame* center_panel = nullptr;
mineral_ui::HeroBannerWidget* hero_banner = nullptr;
QFrame* selected_card = nullptr;
QFrame* lyrics_card = nullptr;
QFrame* spectrum_card = nullptr;
QFrame* transport_card = nullptr;
QWidget* top_bento_area = nullptr;
QWidget* bottom_bento_area = nullptr;
QWidget* close_box = nullptr;
QWidget* queue_tools = nullptr;
QLineEdit* save_playlist_box = nullptr;
QButtonGroup* nav_group = nullptr;
QSystemTrayIcon* tray_icon = nullptr;
QTimer* lyric_anim_timer = nullptr;
QElapsedTimer lyric_dt_timer;
QTimer* sidebar_anim_timer = nullptr;
QElapsedTimer sidebar_dt_timer;
double sidebar_anim_t = 0.0;
double sidebar_anim_vel = 0.0;
double search_focus_t = 0.0;
double search_focus_vel = 0.0;
double filter_focus_t = 0.0;
double filter_focus_vel = 0.0;
bool search_box_focused = false;
bool filter_box_focused = false;
std::vector<QLabel*> side_section_labels;
std::vector<mineral_ui::M3NavRailButton*> side_nav_buttons;
bool sidebar_collapsed = false;
bool now_playing_tab_active = false;
bool play_queue_tab_active = false;
bool first_run_tip_shown = false;
bool allow_quit = false;
int last_generation = -1;
int last_notice_id = -1;
int scheduled_notice_id = -1;
qint64 notice_shown_ms = 0;
bool was_mini = false;
bool was_maximized_before_mini = false;
QSize normal_win_size(1380, 860);
QPoint normal_win_pos(120, 80);
bool normal_pos_valid = false;
QByteArray normal_win_geom;
qint64 suppress_normal_geom_save_until_ms = 0;
QString base_hero_desc;
std::vector<float> viz_peaks;
std::vector<float> viz_peak_holds;
std::vector<float> viz_scope;
std::vector<std::vector<float>> viz_waterfall;
QString viz_style = "bars";
QString close_policy = "ask";
QString current_workspace = "classic";
QString current_content_kind = "tracks";
QString current_theme_bg = "#0d1016";
QString current_theme_fg = "#eef2f6";
QString current_theme_accent = "#6dd5b5";
QString last_play_mode = "sequential";
bool auto_adapt_on = true;
bool locked_on = false;
bool reduce_motion_on = false;
bool daemon_connected = false;
bool touch_on = false;
bool single_mode_on = false;

void push_action(const QString& text) { actions.push_back(text.toStdString()); }

QString escape_json(QString s) {
    s.replace("\\", "\\\\");
    s.replace("\"", "\\\"");
    s.replace("\n", " ");
    return s;
}

QString format_ms(qint64 ms) {
    if (ms <= 0) return QStringLiteral("0:00");
    const qint64 total_sec = ms / 1000;
    const qint64 mins = total_sec / 60;
    const qint64 secs = total_sec % 60;
    return QString("%1:%2").arg(mins).arg(secs, 2, 10, QChar('0'));
}

QString play_mode_text(const QString& mode) {
    if (mode == "repeat_one") return QStringLiteral("Repeat One");
    if (mode == "repeat_all") return QStringLiteral("Repeat All");
    if (mode == "shuffle") return QStringLiteral("Shuffle");
    return QStringLiteral("Sequential");
}

mineral_ui::M3Icon play_mode_icon(const QString& mode) {
    if (mode == "repeat_one") return mineral_ui::M3Icon::ModeRepeatOne;
    if (mode == "repeat_all") return mineral_ui::M3Icon::ModeRepeatAll;
    if (mode == "shuffle") return mineral_ui::M3Icon::ModeShuffle;
    return mineral_ui::M3Icon::ModeSequential;
}

QString lyric_track_text(const QString& track) {
    if (track == "translation") return QStringLiteral("Translation");
    if (track == "romanization") return QStringLiteral("Romanized");
    return QStringLiteral("Bilingual");
}

QString to_english_ui(const QString& s) {
    if (s.isEmpty() || s == "曲库") return QStringLiteral("Browse");
    if (s == "歌单") return QStringLiteral("Playlists");
    if (s == "收藏") return QStringLiteral("Favourites");
    if (s == "下载") return QStringLiteral("Downloads");
    if (s == "队列" || s == "播放队列") return QStringLiteral("Play Queue");
    if (s == "设置") return QStringLiteral("Settings");
    if (s == "搜索") return QStringLiteral("Search");
    if (s == "没有正在播放") return QStringLiteral("Nothing Playing");
    if (s == "还没有收藏") return QStringLiteral("No favourites yet");
    if (s == "还没有下载任务") return QStringLiteral("No downloads yet");
    if (s == "还没有歌单") return QStringLiteral("No playlists yet");
    if (s == "队列还是空的") return QStringLiteral("Play queue is empty");
    if (s == "没有匹配结果" || s == "当前列表没有匹配") return QStringLiteral("No matching results");
    if (s == "这里还是空的") return QStringLiteral("Nothing here yet");
    if (s == "关闭窗口") return QStringLiteral("Close Window Action");
    if (s == "跟随窗口自动切换") return QStringLiteral("Auto-Adapt to Narrow Window");
    if (s == "锁定当前模式") return QStringLiteral("Lock Current Mode");
    if (s == "减少动态效果") return QStringLiteral("Reduce Motion");
    if (s == "重置全部布局") return QStringLiteral("Reset Layout");
    if (s == "恢复默认模式") return QStringLiteral("Reset Modes");
    if (s == "不影响播放队列") return QStringLiteral("Preserves current play queue");
    if (s == "开") return QStringLiteral("On");
    if (s == "关") return QStringLiteral("Off");
    if (s == "已锁定") return QStringLiteral("Locked");
    if (s == "未锁定") return QStringLiteral("Unlocked");
    if (s == "ask") return QStringLiteral("Ask every time");
    if (s == "ui") return QStringLiteral("Keep playing in System Tray");
    if (s == "stop") return QStringLiteral("Stop playback & quit");
    if (s == "已连接") return QStringLiteral("Connected");
    if (s == "未连接") return QStringLiteral("Disconnected");
    if (s == "正在重连") return QStringLiteral("Reconnecting...");
    if (s == "正在启动 daemon") return QStringLiteral("Launching daemon...");
    if (s == "播放已停止") return QStringLiteral("Playback stopped");
    if (s.startsWith("已加入队列")) return QStringLiteral("Added to Play Queue");
    if (s.startsWith("将在当前歌曲后播放")) return QStringLiteral("Will play next");
    if (s.startsWith("已提交下载")) return QStringLiteral("Download started");
    if (s.startsWith("已重置布局")) return QStringLiteral("Layout reset");
    if (s.startsWith("已恢复默认模式")) return QStringLiteral("Default mode restored");
    return s;
}

QIcon make_tray_icon() {
    QPixmap src;
    if (src.loadFromData(mineral_ui::MINERAL_LOGO_JPG, static_cast<uint>(mineral_ui::MINERAL_LOGO_JPG_LEN), "JPG") && !src.isNull()) {
        QIcon icon;
        for (int sz : {64, 128, 256}) {
            QPixmap scaled = src.scaled(sz, sz, Qt::KeepAspectRatioByExpanding, Qt::SmoothTransformation);
            QPixmap out(sz, sz);
            out.fill(Qt::transparent);
            QPainter p(&out);
            p.setRenderHint(QPainter::Antialiasing, true);
            p.setRenderHint(QPainter::SmoothPixmapTransform, true);
            QPainterPath clip;
            const qreal rad = sz * 0.22;
            clip.addRoundedRect(QRectF(0.0, 0.0, sz, sz), rad, rad);
            p.setClipPath(clip);
            p.drawPixmap(0, 0, scaled);
            icon.addPixmap(out);
        }
        return icon;
    }
    QPixmap pm(64, 64);
    pm.fill(Qt::transparent);
    QPainter p(&pm);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.setPen(Qt::NoPen);
    p.setBrush(mineral_ui::m3.primary);
    p.drawEllipse(QRectF(4.0, 4.0, 56.0, 56.0));
    mineral_ui::paint_md3_icon(&p, mineral_ui::M3Icon::Play, QRectF(14.0, 14.0, 36.0, 36.0), QColor("#FFFFFF"));
    return QIcon(pm);
}

void hide_to_system_tray() {
    if (tray_icon) {
        tray_icon->setIcon(make_tray_icon());
        if (!tray_icon->isVisible()) tray_icon->show();
        if (QSystemTrayIcon::supportsMessages()) {
            tray_icon->showMessage(
                QStringLiteral("Mineral"),
                QStringLiteral("Playback continues in the background. Click the system tray icon to restore."),
                QSystemTrayIcon::Information,
                2800
            );
        }
    }
    if (close_box) close_box->setVisible(false);
    if (window) window->hide();
}

void restore_from_system_tray() {
    if (!window) return;
    window->show();
    if (was_mini) {
        window->setMinimumSize(640, 260);
        window->setMaximumSize(700, 280);
        window->resize(650, 268);
    } else if (was_maximized_before_mini) {
        window->showMaximized();
    } else {
        window->showNormal();
    }
    window->raise();
    window->activateWindow();
}

void sync_active_lyric_from_pos(qint64 pos_ms);

qint64 viz_pos_ms = 0;
qint64 viz_dur_ms = 0;
bool last_playing_state = false;
bool last_loved_state = false;

double current_screen_refresh_hz(QWidget* w = nullptr) {
    QScreen* scr = nullptr;
    if (w && w->screen()) {
        scr = w->screen();
    } else if (window && window->screen()) {
        scr = window->screen();
    } else {
        scr = QGuiApplication::primaryScreen();
    }
    double hz = scr ? static_cast<double>(scr->refreshRate()) : 60.0;
    if (!std::isfinite(hz) || hz < 24.0) hz = 60.0;
    if (hz > 500.0) hz = 500.0;
    return hz;
}

int current_screen_frame_interval_ms(QWidget* w = nullptr) {
    const double hz = current_screen_refresh_hz(w);
    return std::max(1, static_cast<int>(std::floor(1000.0 / hz)));
}

void sync_timer_to_screen_refresh(QTimer* timer, QWidget* w = nullptr) {
    if (!timer) return;
    const int target_ms = current_screen_frame_interval_ms(w);
    if (timer->interval() != target_ms) {
        timer->setInterval(target_ms);
    }
}

class VizWidget : public QWidget {
    std::vector<float> smooth_bars_;
    float phase_ = 0.0f;
    double pos_accum_ms_ = 0.0;
    int settle_ticks_ = 0;
    QElapsedTimer dt_timer_;
    QTimer* anim_timer_ = nullptr;

    float baseEnvelope(int i) const {
        return 0.14f
             + 0.42f * std::abs(std::sin(i * 0.47f + 0.8f) * std::cos(i * 0.21f - 0.4f))
             + 0.28f * std::abs(std::sin(i * 0.93f + 1.5f));
    }

public:
    explicit VizWidget(QWidget* parent = nullptr) : QWidget(parent), smooth_bars_(96, 0.2f) {
        setCursor(Qt::PointingHandCursor);
        setToolTip("Click or drag soundwave to scrub playback position");
        for (int i = 0; i < 96; ++i) {
            smooth_bars_[i] = baseEnvelope(i);
        }
        anim_timer_ = new QTimer(this);
        anim_timer_->setTimerType(Qt::PreciseTimer);
        sync_timer_to_screen_refresh(anim_timer_, this);
        QObject::connect(anim_timer_, &QTimer::timeout, this, [this] { tickFluid(); });
    }

    void notifyPlaybackState(bool playing) {
        sync_timer_to_screen_refresh(anim_timer_, this);
        if (playing) {
            settle_ticks_ = static_cast<int>(std::ceil(current_screen_refresh_hz(this) * 0.5));
            dt_timer_.restart();
            if (!anim_timer_->isActive()) anim_timer_->start();
        } else if (settle_ticks_ <= 0 && anim_timer_->isActive()) {
            anim_timer_->stop();
        }
    }

protected:
    void tickFluid() {
        sync_timer_to_screen_refresh(anim_timer_, this);
        const double dt_sec = dt_timer_.isValid()
            ? std::clamp(double(dt_timer_.nsecsElapsed()) * 1e-9, 0.0005, 0.050)
            : (1.0 / current_screen_refresh_hz(this));
        dt_timer_.restart();
        if (last_playing_state && viz_dur_ms > 0 && !reduce_motion_on) {
            pos_accum_ms_ += dt_sec * 1000.0;
            const qint64 step_ms = static_cast<qint64>(pos_accum_ms_);
            if (step_ms > 0) {
                pos_accum_ms_ -= double(step_ms);
                viz_pos_ms = std::min(viz_dur_ms, viz_pos_ms + step_ms);
                sync_active_lyric_from_pos(viz_pos_ms);
            }
        }
        if (!isVisible()) return;
        const float dt_scale = static_cast<float>(dt_sec * 30.0);
        if (last_playing_state && !reduce_motion_on) {
            phase_ += 0.16f * dt_scale;
            if (phase_ > 628.0f) phase_ -= 628.0f;
            settle_ticks_ = static_cast<int>(std::ceil(current_screen_refresh_hz(this) * 0.5));
        } else {
            if (--settle_ticks_ <= 0) {
                anim_timer_->stop();
            }
        }

        const float attack_alpha = 1.0f - std::pow(1.0f - 0.36f, dt_scale);
        const float decay_alpha = 1.0f - std::pow(1.0f - 0.18f, dt_scale);
        const int n = static_cast<int>(smooth_bars_.size());
        for (int i = 0; i < n; ++i) {
            const float t = float(i) / float(std::max(1, n - 1));
            float target = baseEnvelope(i);
            if (!viz_peaks.empty()) {
                const float exact_idx = t * float(viz_peaks.size() - 1);
                const size_t idx0 = static_cast<size_t>(exact_idx);
                const size_t idx1 = std::min(idx0 + 1, viz_peaks.size() - 1);
                const float frac = exact_idx - float(idx0);
                const float p0 = std::clamp(std::abs(viz_peaks[idx0]), 0.0f, 1.0f);
                const float p1 = std::clamp(std::abs(viz_peaks[idx1]), 0.0f, 1.0f);
                const float live = p0 + (p1 - p0) * frac;
                target = target * 0.52f + live * 0.62f;
            }
            if (last_playing_state && !reduce_motion_on) {
                const float wave = 0.16f * std::sin(phase_ * 1.25f + i * 0.36f)
                                 * std::cos(phase_ * 0.68f - i * 0.19f);
                target = std::clamp(target + wave, 0.08f, 0.98f);
            }
            const float cur = smooth_bars_[i];
            const float alpha = (target > cur) ? attack_alpha : decay_alpha;
            smooth_bars_[i] = cur + (target - cur) * alpha;
        }
        update();
    }

    void seekAt(int mx, bool commit) {
        const int w = std::max(1, width() - 8);
        const double ratio = std::clamp(double(mx - 4) / double(w), 0.0, 1.0);
        if (viz_dur_ms > 0) {
            viz_pos_ms = static_cast<qint64>(ratio * viz_dur_ms);
            sync_active_lyric_from_pos(viz_pos_ms);
            if (position && !position->isSliderDown()) {
                position->setValue(static_cast<int>(viz_pos_ms));
            }
            if (pos_time_label) {
                pos_time_label->setText(format_ms(viz_pos_ms));
            }
            if (commit) {
                push_action(QString("{\"action\":\"seek\",\"ms\":%1}").arg(viz_pos_ms));
            }
        }
        update();
    }
    void mousePressEvent(QMouseEvent* event) override {
        if (event->button() == Qt::LeftButton) {
            seekAt(event->pos().x(), true);
        }
    }
    void mouseMoveEvent(QMouseEvent* event) override {
        if (event->buttons() & Qt::LeftButton) {
            seekAt(event->pos().x(), false);
        }
    }
    void mouseReleaseEvent(QMouseEvent* event) override {
        if (event->button() == Qt::LeftButton) {
            seekAt(event->pos().x(), true);
        }
    }
    void paintEvent(QPaintEvent*) override {
        QPainter painter(this);
        painter.setRenderHint(QPainter::Antialiasing);

        const int w = width();
        const int h = height();
        if (w <= 10 || h <= 8) return;

        const float mid_y = h * 0.5f;
        const int num_bars = std::clamp(w / 5, 52, 96);
        const float step_x = float(w - 4) / float(num_bars);
        const float progress = (viz_dur_ms > 0)
            ? std::clamp(float(viz_pos_ms) / float(viz_dur_ms), 0.0f, 1.0f)
            : 0.24f;

        // Subtle center baseline
        painter.setPen(Qt::NoPen);
        painter.setBrush(QColor(200, 184, 175, 90));
        painter.drawRect(QRectF(2.0f, mid_y - 0.5f, w - 4.0f, 1.0f));

        for (int i = 0; i < num_bars; ++i) {
            const float t = float(i) / float(std::max(1, num_bars - 1));
            const float edge_taper = std::min({1.0f, t * 8.0f, (1.0f - t) * 8.0f});
            const size_t sb_idx = std::min<size_t>(
                smooth_bars_.size() - 1,
                static_cast<size_t>(t * (smooth_bars_.size() - 1)));
            const float env = smooth_bars_[sb_idx];
            const float norm_h = std::clamp(env * edge_taper, 0.08f, 0.95f);
            const float bar_h = std::max(3.0f, norm_h * (h - 4.0f));
            const float x = 2.0f + i * step_x;

            const bool is_played = (t <= progress);
            painter.setBrush(is_played ? mineral_ui::m3.waveActive : mineral_ui::m3.waveInactive);
            painter.drawRoundedRect(QRectF(x, mid_y - bar_h * 0.5f, 2.0f, bar_h), 1.0f, 1.0f);
        }
    }
};
VizWidget* viz_widget = nullptr;

void adjust_lyric_font_size(int delta);

class Window : public QMainWindow {
    int last_narrow_flag = -1;
    int last_typing_flag = -1;
protected:
    void closeEvent(QCloseEvent* event) override {
        if (allow_quit) {
            event->accept();
            return;
        }
        if (close_policy == "ui") {
            hide_to_system_tray();
            event->ignore();
            return;
        }
        push_action("{\"action\":\"request_close\"}");
        event->ignore();
    }
    void saveNormalGeometryIfStable() {
        if (!was_mini && !isFullScreen() && !isMaximized() &&
            QDateTime::currentMSecsSinceEpoch() >= suppress_normal_geom_save_until_ms &&
            width() >= 860 && height() >= 560) {
            normal_win_size = size();
            normal_win_pos = pos();
            normal_pos_valid = true;
            normal_win_geom = saveGeometry();
        }
    }
    void changeEvent(QEvent* event) override {
        QMainWindow::changeEvent(event);
        if (event && event->type() == QEvent::WindowStateChange && was_mini) {
            if (isFullScreen() || isMaximized()) {
                QTimer::singleShot(0, this, [this] {
                    if (was_mini) {
                        setWindowState(windowState() & ~(Qt::WindowFullScreen | Qt::WindowMaximized));
                        showNormal();
                        setFixedSize(650, 268);
                    }
                });
            }
        }
    }
    void moveEvent(QMoveEvent* event) override {
        QMainWindow::moveEvent(event);
        saveNormalGeometryIfStable();
    }
    void resizeEvent(QResizeEvent* event) override {
        QMainWindow::resizeEvent(event);
        saveNormalGeometryIfStable();
    }
    void keyPressEvent(QKeyEvent* event) override {
        const bool typing = (search_box && search_box->hasFocus()) || (filter_box && filter_box->hasFocus()) ||
                            (inline_filter_box && inline_filter_box->hasFocus()) ||
                            (save_playlist_box && save_playlist_box->hasFocus());
        const int typing_flag = typing ? 1 : 0;
        if (typing_flag != last_typing_flag) {
            last_typing_flag = typing_flag;
            push_action(QString("{\"action\":\"set_typing\",\"on\":%1}").arg(typing ? "true" : "false"));
        }
        const auto key = event->key();
        const bool ctrl = event->modifiers().testFlag(Qt::ControlModifier);
        const bool shift = event->modifiers().testFlag(Qt::ShiftModifier);
        if (ctrl && key == Qt::Key_K && !shift) push_action("{\"action\":\"palette\"}");
        else if (!typing && (key == Qt::Key_Plus || key == Qt::Key_Equal)) adjust_lyric_font_size(2);
        else if (!typing && key == Qt::Key_Minus) adjust_lyric_font_size(-2);
        else if (key == Qt::Key_Escape) push_action("{\"action\":\"back\"}");
        else if (!typing && key == Qt::Key_Question) push_action("{\"action\":\"help\",\"open\":true}");
        else if (!typing && key == Qt::Key_Space) push_action("{\"action\":\"toggle_play\"}");
        else if (!typing && (key == Qt::Key_Return || key == Qt::Key_Enter) && shift) push_action("{\"action\":\"enqueue\",\"key\":\"\"}");
        else if (!typing && (key == Qt::Key_Return || key == Qt::Key_Enter)) push_action("{\"action\":\"activate\",\"key\":\"\"}");
        else if (!typing && key == Qt::Key_J && !ctrl) push_action("{\"action\":\"select_delta\",\"delta\":1}");
        else if (!typing && key == Qt::Key_K && !ctrl) push_action("{\"action\":\"select_delta\",\"delta\":-1}");
        else if (!typing && key == Qt::Key_N) push_action("{\"action\":\"next\"}");
        else if (!typing && key == Qt::Key_P) push_action("{\"action\":\"prev\"}");
        else if (!typing && key == Qt::Key_L) push_action("{\"action\":\"love\",\"key\":\"\"}");
        else if (!typing && key == Qt::Key_D) push_action("{\"action\":\"download\",\"key\":\"\"}");
        else if (was_mini && (key == Qt::Key_Z || key == Qt::Key_F11)) { event->accept(); return; }
        else if (!typing && key == Qt::Key_Z) push_action("{\"action\":\"toggle_immersive\"}");
        else if (!typing && key == Qt::Key_T) push_action("{\"action\":\"lyric_track\"}");
        else if (!typing && key == Qt::Key_C) push_action("{\"action\":\"reveal\"}");
        else if (!typing && key == Qt::Key_M) push_action("{\"action\":\"cycle_mode\"}");
        else if (!typing && key == Qt::Key_Tab) push_action("{\"action\":\"queue_drawer\"}");
        else if (!typing && key == Qt::Key_Slash) { if (inline_filter_box) inline_filter_box->setFocus(); }
        else if (!typing && ctrl && key == Qt::Key_J) push_action("{\"action\":\"queue_move_selected\",\"to\":\"down\"}");
        else if (!typing && shift && key == Qt::Key_Up) push_action("{\"action\":\"queue_move_selected\",\"to\":\"up\"}");
        else if (!typing && shift && key == Qt::Key_Down) push_action("{\"action\":\"queue_move_selected\",\"to\":\"down\"}");
        else if (!typing && key == Qt::Key_Delete) push_action("{\"action\":\"queue_remove_selected\"}");
        else {
            QMainWindow::keyPressEvent(event);
            return;
        }
        event->accept();
    }
};

class SearchFocusEventFilter : public QObject {
public:
    explicit SearchFocusEventFilter(QObject* parent = nullptr) : QObject(parent) {}
protected:
    bool eventFilter(QObject* watched, QEvent* event) override {
        if (!event) return QObject::eventFilter(watched, event);
        const auto type = event->type();
        auto start_anim = [] {
            sidebar_dt_timer.restart();
            if (sidebar_anim_timer && !sidebar_anim_timer->isActive()) {
                sidebar_anim_timer->start();
            }
        };
        if (watched == search_bar_frame_ref && type == QEvent::MouseButtonPress) {
            if (sidebar_collapsed) {
                sidebar_collapsed = false;
                if (sidebar_toggle_btn) {
                    sidebar_toggle_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::SidebarCollapse, QColor("#FFFFFF"), 16));
                }
            }
            if (search_box) search_box->setFocus();
            start_anim();
        } else if (watched == search_box) {
            if (type == QEvent::FocusIn || type == QEvent::MouseButtonPress) {
                if (sidebar_collapsed) {
                    sidebar_collapsed = false;
                    if (sidebar_toggle_btn) {
                        sidebar_toggle_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::SidebarCollapse, QColor("#FFFFFF"), 16));
                    }
                }
                if (!search_box_focused) {
                    search_box_focused = true;
                    start_anim();
                }
            } else if (type == QEvent::FocusOut) {
                if (search_box_focused) {
                    search_box_focused = false;
                    start_anim();
                }
            }
        } else if (watched == inline_filter_box) {
            if (type == QEvent::FocusIn || type == QEvent::MouseButtonPress) {
                if (!filter_box_focused) {
                    filter_box_focused = true;
                    start_anim();
                }
            } else if (type == QEvent::FocusOut) {
                if (filter_box_focused) {
                    filter_box_focused = false;
                    start_anim();
                }
            }
        }
        return QObject::eventFilter(watched, event);
    }
};

QString shell_style(bool touch = false) {
    const auto& s = mineral_ui::m3;
    QString base = QString(
        "QMainWindow, QDialog, QWidget#root { background:%1; color:%2; font-family:'Noto Sans CJK SC','Google Sans','Inter','Segoe UI',sans-serif; }"
        "QWidget#navRail { background:%12; color:#FFFFFF; border:none; }"
        "QWidget#topbar { background:%1; border:none; }"
        "QFrame#searchBar { background:%13; border:none; border-radius:18px; }"
        "QFrame#bentoCard { background:%1; border:none; border-radius:0px; }"
        "QFrame#heroCard { background:transparent; border:none; border-radius:0px; }"
        "QFrame#transportCard { background:transparent; border:none; }"
        "QWidget#subheader { background:%4; border:none; border-radius:10px; }"
        "QWidget#colheader { background:transparent; border-bottom:1px solid #F2EFEC; border-radius:0px; }"
        "QWidget#notice { background:%9; color:%10; border:none; border-radius:10px; }"
        "QSplitter::handle { background:transparent; width:1px; height:1px; }"
        "QLabel { color:%2; background:transparent; }"
        "QLabel#sideSection { color:rgba(255,255,255,180); font-size:10px; font-weight:700; letter-spacing:1.2px; padding-left:10px; }"
        "QLabel#brand { color:#FFFFFF; background:rgba(255,255,255,38); border-radius:10px; padding:4px 10px; font-size:12px; font-weight:800; }"
        "QLabel#cardtag { color:%2; font-size:18px; font-weight:700; }"
        "QLabel#muted { color:%3; font-size:12px; }"
        "QLabel#collabel { color:%3; font-size:11px; font-weight:500; }"
        "QLabel#heroTime { color:#FFFFFF; font-size:11px; font-weight:700; }"
        "QLabel#heroTitle { color:#FFFFFF; font-size:38px; font-weight:800; }"
        "QLabel#heroDesc { color:#FAF6F4; font-size:12px; font-weight:500; }"
        "QLabel#daemon { color:rgba(255,255,255,220); background:rgba(255,255,255,32); border:none; border-radius:11px; padding:4px 10px; font-size:11px; font-weight:600; }"
        "QLabel#crumbs { color:%7; font-size:12px; font-weight:600; }"
        "QLabel#heading { font-size:21px; font-weight:700; color:%2; }"
        "QLabel#seltitle { font-size:18px; font-weight:700; color:%2; }"
        "QLabel#nowtitle { font-size:36px; font-weight:800; color:#FFFFFF; }"
        "QLineEdit { background:%4; border:1px solid %11; border-radius:16px; padding:6px 14px; color:%2; font-size:12px; selection-background-color:%7; selection-color:%8; }"
        "QLineEdit:focus { background:%1; border:1px solid %7; }"
        "QLineEdit#searchInput { background:transparent; border:none; padding:0px 2px; margin:0px; font-size:12px; font-weight:500; color:#FFFFFF; selection-background-color:rgba(255,255,255,80); selection-color:#FFFFFF; }"
        "QLineEdit#searchInput:focus { background:transparent; border:none; }"
        "QPushButton { background:transparent; color:%2; border:none; border-radius:13px; padding:5px 12px; font-weight:600; font-size:12px; }"
        "QPushButton:hover { background:%5; }"
        "QPushButton#backLink { background:transparent; color:#FFFFFF; border:none; padding:2px 0px; font-size:12px; font-weight:600; text-align:left; }"
        "QPushButton#backLink:hover { color:%12; background:transparent; }"
        "QPushButton#heroTag { background:rgba(255,255,255,52); color:#FFFFFF; border:none; border-radius:13px; padding:4px 14px; font-size:11px; font-weight:600; }"
        "QPushButton#heroTag:hover { background:%12; color:#FFFFFF; }"
        "QPushButton#chip { background:%4; color:%3; border:none; border-radius:12px; padding:4px 12px; font-size:11px; font-weight:600; }"
        "QPushButton#chip:hover { background:%5; color:%2; }"
        "QPushButton#chip:checked { background:%9; color:%10; border:none; border-radius:12px; font-weight:700; }"
        "QPushButton#mode { background:%9; color:%10; border:none; border-radius:13px; padding:5px 14px; font-weight:600; font-size:12px; }"
        "QPushButton#play { background:%12; color:#FFFFFF; border:none; border-radius:26px; min-width:52px; max-width:52px; min-height:52px; max-height:52px; padding:0px; }"
        "QPushButton#play:hover { background:%7; color:#FFFFFF; }"
        "QPushButton#ghost { background:%9; color:%10; border:none; border-radius:13px; padding:5px 12px; font-size:11px; font-weight:600; }"
        "QPushButton#ghost:hover { background:%6; color:%2; }"
        "QPushButton#searchBtn { background:transparent; color:rgba(255,255,255,230); border:none; border-radius:12px; min-width:24px; max-width:24px; min-height:24px; max-height:24px; padding:0px; margin:0px; }"
        "QPushButton#searchBtn:hover { background:rgba(255,255,255,40); color:#FFFFFF; }"
        "QPushButton#iconBtn { background:transparent; color:rgba(255,255,255,220); border:none; border-radius:15px; min-width:30px; min-height:30px; padding:3px; }"
        "QPushButton#iconBtn:hover { background:rgba(255,255,255,40); color:#FFFFFF; }"
        "QPushButton#heroIconBtn { background:rgba(255,255,255,55); color:#FFFFFF; border:none; border-radius:14px; min-width:28px; max-width:28px; min-height:28px; max-height:28px; padding:0px; }"
        "QPushButton#heroIconBtn:hover { background:%12; color:#FFFFFF; }"
        "QListWidget { background:transparent; border:none; color:%2; outline:none; }"
        "QListWidget::item { padding:0px; border:none; background:transparent; }"
        "QListWidget::item:hover { background:transparent; }"
        "QListWidget::item:selected { background:transparent; color:%2; }"
        "QScrollBar:vertical { background:#FFFFFF; width:6px; margin:0px; border:none; }"
        "QScrollBar::handle:vertical { background:#E2DDD9; border-radius:3px; min-height:32px; border:none; }"
        "QScrollBar::handle:vertical:hover { background:%12; }"
        "QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical { height:0px; width:0px; border:none; background:#FFFFFF; }"
        "QScrollBar::add-page:vertical, QScrollBar::sub-page:vertical { background:#FFFFFF; border:none; }"
        "QSlider { background:transparent; border:none; }"
        "QSlider::groove:horizontal { height:4px; background:rgba(255,255,255,95); border-radius:2px; border:none; }"
        "QSlider::sub-page:horizontal { background:%12; border-radius:2px; border:none; }"
        "QSlider::add-page:horizontal { background:rgba(255,255,255,95); border-radius:2px; border:none; }"
        "QSlider::handle:horizontal { width:10px; height:10px; margin:-3px 0; background:#FFFFFF; border:2px solid %12; border-radius:5px; }"
        "QMenu { background:#FFFFFF; color:%2; border:1px solid %11; border-radius:12px; padding:6px; }"
        "QMenu::item { padding:7px 16px; border-radius:8px; font-size:12px; }"
        "QMenu::item:selected { background:%9; color:%10; }"
    )
    .arg(s.surface.name(),                 // %1  #FFFFFF
         s.onSurface.name(),               // %2  #2B2625
         s.onSurfaceVariant.name(),        // %3  #6E6662
         s.surfaceContainerLow.name(),     // %4  #FAF8F7
         s.surfaceContainerHigh.name(),    // %5  #EFEAE6
         s.surfaceContainerHighest.name(), // %6  #E6DFDA
         s.primaryDark.name(),             // %7  #E07A5F
         s.onPrimary.name(),               // %8  #FFFFFF
         s.secondaryContainer.name())      // %9  #F7EAE5
    .arg(s.onSecondaryContainer.name(),    // %10 #5C3B31
         s.outlineVariant.name(),          // %11 #F0ECE9
         s.primary.name(),                 // %12 #EA957A
         s.primaryContainer.name());       // %13 #F2AE98
    if (touch) {
        base += "QPushButton { min-height:38px; padding:6px 14px; font-size:13px; }";
    }
    return base;
}

void fill_list(QListWidget* list, const QJsonArray& array, const QString& selected, bool grid_mode = false) {
    QSignalBlocker blocker(list);

    // In Cover Grid mode, filter out source status header rows ("source:netease", etc.) when actual tracks/playlists/albums exist
    QJsonArray effective_array;
    if (grid_mode) {
        for (const auto& value : array) {
            if (value.toObject().value("kind").toString() != "source") {
                effective_array.append(value);
            }
        }
        if (effective_array.isEmpty()) {
            effective_array = array;
        }
    } else {
        effective_array = array;
    }

    QString sig = grid_mode ? QStringLiteral("G|") : QStringLiteral("L|");
    sig.reserve(effective_array.size() * 64);
    for (const auto& value : effective_array) {
        const auto row = value.toObject();
        const auto cover = row.value("cover_url").toString();
        sig += row.value("key").toString();
        sig += QChar('\x1f');
        sig += row.value("title").toString();
        sig += QChar('\x1f');
        sig += row.value("subtitle").toString();
        sig += QChar('\x1f');
        sig += row.value("album").toString();
        sig += QChar('\x1f');
        sig += row.value("current").toBool() ? QChar('1') : QChar('0');
        sig += row.value("loved").toBool() ? QChar('1') : QChar('0');
        sig += (!cover.isEmpty() && QFileInfo::exists(cover)) ? QChar('C') : QChar('0');
        sig += QChar('\n');
    }
    if (list->property("content_sig").toString() == sig) {
        if (list->property("sel_key").toString() != selected) {
            list->setProperty("sel_key", selected);
            for (int i = 0; i < list->count(); ++i) {
                auto* item = list->item(i);
                const bool match = item->data(mineral_ui::RoleKey).toString() == selected;
                if (item->isSelected() != match) {
                    item->setSelected(match);
                }
            }
            if (list->viewport()) list->viewport()->update();
        }
        return;
    }
    list->setProperty("content_sig", sig);
    list->setProperty("sel_key", selected);

    const bool same_mode_and_count =
        (list->count() == effective_array.size()) &&
        (list->property("grid_mode").isValid() && list->property("grid_mode").toBool() == grid_mode);
    list->setProperty("grid_mode", grid_mode);

    const int old_scroll = list->verticalScrollBar() ? list->verticalScrollBar()->value() : 0;

    auto populate_item = [&](QListWidgetItem* item, const QJsonObject& row) {
        const auto key = row.value("key").toString();
        const auto kind = row.value("kind").toString();
        const bool is_setting = (kind == "setting" || key.startsWith("settings:"));
        const auto title = is_setting ? to_english_ui(row.value("title").toString()) : row.value("title").toString();
        const auto sub = is_setting ? to_english_ui(row.value("subtitle").toString()) : row.value("subtitle").toString();
        const auto album = row.value("album").toString();
        const auto source = row.value("source").toString();
        const auto cover = row.value("cover_url").toString();
        const bool current = row.value("current").toBool();
        const bool loved = row.value("loved").toBool();
        const qint64 dur_ms = row.value("duration_ms").toVariant().toLongLong();
        const QString dur_str = dur_ms > 0 ? format_ms(dur_ms) : QString();

        item->setData(mineral_ui::RoleKey, key);
        item->setData(mineral_ui::RoleKind, kind);
        item->setData(mineral_ui::RoleTitle, title);
        item->setData(mineral_ui::RoleSubtitle, sub);
        item->setData(mineral_ui::RoleAlbum, album);
        item->setData(mineral_ui::RoleSource, source);
        item->setData(mineral_ui::RoleDuration, dur_str);
        item->setData(mineral_ui::RoleCurrent, current);
        item->setData(mineral_ui::RoleLoved, loved);
        item->setData(mineral_ui::RoleCover, cover);
        item->setData(mineral_ui::RoleGridMode, grid_mode);
        if (row.value("index").isDouble()) {
            item->setData(mineral_ui::RoleIndex, row.value("index").toInt());
        } else {
            item->setData(mineral_ui::RoleIndex, QVariant());
        }

        if (grid_mode) {
            item->setText(QString());
            item->setToolTip(title + (sub.isEmpty() ? "" : (" — " + sub)));
            item->setSizeHint(QSize(164, 164));
        } else {
            item->setText(title + " " + sub);
            item->setToolTip(title + (sub.isEmpty() ? "" : (" — " + sub)) + (album.isEmpty() ? "" : (" · " + album)));
            item->setSizeHint(QSize(0, 56));
        }
        const bool is_sel = (key == selected);
        if (item->isSelected() != is_sel) {
            item->setSelected(is_sel);
        }
    };

    if (same_mode_and_count) {
        // Update items in-place without clearing or calling setCurrentItem so scroll position never jumps!
        for (int i = 0; i < effective_array.size(); ++i) {
            populate_item(list->item(i), effective_array.at(i).toObject());
        }
        if (list->verticalScrollBar()) {
            list->verticalScrollBar()->setValue(old_scroll);
        }
        if (list->viewport()) list->viewport()->update();
        return;
    }

    list->clear();
    if (grid_mode) {
        list->setViewMode(QListWidget::IconMode);
        list->setIconSize(QSize(0, 0));
        list->setGridSize(QSize(168, 168));
        list->setResizeMode(QListWidget::Adjust);
        list->setMovement(QListView::Static);
        list->setMouseTracking(true);
        list->setWordWrap(false);
    } else {
        list->setViewMode(QListWidget::ListMode);
        list->setIconSize(QSize(0, 0));
        list->setGridSize(QSize());
        list->setWordWrap(false);
    }
    for (const auto& value : effective_array) {
        auto* item = new QListWidgetItem();
        populate_item(item, value.toObject());
        list->addItem(item);
    }
    list->doItemsLayout();
    if (list->verticalScrollBar()) {
        list->verticalScrollBar()->setValue(old_scroll);
    }
    if (list->viewport()) list->viewport()->update();
}

QPushButton* prev_btn_ref = nullptr;
QPushButton* next_btn_ref = nullptr;
QPushButton* queue_btn_ref = nullptr;
QPushButton* mini_btn_ref = nullptr;
QPushButton* immersive_btn_ref = nullptr;
QPushButton* play_all_btn_ref = nullptr;
QPushButton* sel_play_btn_ref = nullptr;
QPushButton* sel_enq_btn_ref = nullptr;
QPushButton* sel_dl_btn_ref = nullptr;
QLabel* search_icon_label = nullptr;

// Single-line auto-eliding label (`...` on overflow, never wraps to 2 lines, never clips vertically)
class ElidedLabel : public QLabel {
public:
    explicit ElidedLabel(const QString& text = QString(), QWidget* parent = nullptr)
        : QLabel(text, parent) {
        setWordWrap(false);
        setMinimumWidth(0);
        setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
    }

    QSize minimumSizeHint() const override {
        return QSize(0, fontMetrics().height() + 6);
    }

    QSize sizeHint() const override {
        return QSize(120, fontMetrics().height() + 6);
    }

protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.setRenderHint(QPainter::TextAntialiasing, true);
        p.setFont(font());
        p.setPen(palette().color(foregroundRole()));
        const QRect r = contentsRect();
        if (r.width() < 6) return;
        const QString raw = text().simplified();
        const QString elided = fontMetrics().elidedText(raw, Qt::ElideRight, r.width());
        p.drawText(r, static_cast<int>((alignment() & Qt::AlignHorizontal_Mask) | Qt::AlignVCenter | Qt::TextSingleLine), elided);
    }
};

// Non-linear inertial wheel/trackpad scroller with flick acceleration, smooth attack, and power-law ease-out tail
class InertialWheelScroller : public QObject {
    QListWidget* list_ = nullptr;
    bool is_lyric_ = false;
    QTimer* timer_ = nullptr;
    QElapsedTimer dt_timer_;
    QElapsedTimer wheel_gap_timer_;
    double pos_y_ = 0.0;
    double vel_y_ = 0.0;
    double pend_impulse_ = 0.0;
    double flick_gain_ = 1.0;
    int last_sb_int_ = -999999;

public:
    explicit InertialWheelScroller(QListWidget* list, bool is_lyric = false)
        : QObject(list), list_(list), is_lyric_(is_lyric) {
        if (!list_) return;
        list_->installEventFilter(this);
        if (list_->viewport()) {
            list_->viewport()->installEventFilter(this);
        }
        timer_ = new QTimer(this);
        timer_->setTimerType(Qt::PreciseTimer);
        sync_timer_to_screen_refresh(timer_, list_);
        QObject::connect(timer_, &QTimer::timeout, this, [this] { tickPhysics(); });
    }

protected:
    bool eventFilter(QObject* watched, QEvent* event) override {
        if (event->type() == QEvent::Wheel && list_ && !reduce_motion_on) {
            auto* sb = list_->verticalScrollBar();
            if (!sb || sb->maximum() <= sb->minimum()) {
                return QObject::eventFilter(watched, event);
            }
            auto* we = static_cast<QWheelEvent*>(event);
            double delta_px = 0.0;
            if (!we->pixelDelta().isNull()) {
                delta_px = -double(we->pixelDelta().y());
            } else {
                delta_px = -double(we->angleDelta().y()) * 0.65;
            }
            if (std::abs(delta_px) < 0.01) {
                return QObject::eventFilter(watched, event);
            }

            if (!timer_->isActive() || std::abs(sb->value() - last_sb_int_) > 2) {
                pos_y_ = list_->property("smooth_scroll_y").isValid() && std::abs(list_->property("smooth_scroll_y").toDouble() - double(sb->value())) <= 2.5
                    ? list_->property("smooth_scroll_y").toDouble()
                    : double(sb->value());
                last_sb_int_ = sb->value();
            }

            const qint64 gap_ms = wheel_gap_timer_.isValid() ? wheel_gap_timer_.restart() : 500;
            if ((delta_px * vel_y_) < -1.0 || (delta_px * pend_impulse_) < -1.0) {
                // Crisp direction reversal
                vel_y_ *= 0.12;
                pend_impulse_ = 0.0;
                flick_gain_ = 1.0;
            } else if (gap_ms < 95) {
                // Non-linear flick boost on rapid wheel spins
                const double speed_factor = std::clamp(std::pow(std::abs(vel_y_) / 750.0, 0.6), 0.0, 1.4);
                flick_gain_ = std::min(2.75, flick_gain_ + 0.24 + speed_factor * 0.12);
            } else if (gap_ms > 190) {
                flick_gain_ = 1.0;
            }

            pend_impulse_ += delta_px * flick_gain_;
            if (is_lyric_) {
                list_->setProperty("user_wheel_until_ms", QDateTime::currentMSecsSinceEpoch() + 2600);
            }
            if (!timer_->isActive()) {
                sync_timer_to_screen_refresh(timer_, list_);
                dt_timer_.restart();
                timer_->start();
            }
            we->accept();
            return true;
        }
        return QObject::eventFilter(watched, event);
    }

private:
    void tickPhysics() {
        if (!list_ || !list_->verticalScrollBar()) {
            timer_->stop();
            return;
        }
        sync_timer_to_screen_refresh(timer_, list_);
        auto* sb = list_->verticalScrollBar();
        const double dt = dt_timer_.isValid()
            ? std::clamp(double(dt_timer_.nsecsElapsed()) * 1e-9, 0.0005, 0.048)
            : (1.0 / current_screen_refresh_hz(list_));
        dt_timer_.restart();

        if (std::abs(sb->value() - last_sb_int_) > 2) {
            pos_y_ = double(sb->value());
            last_sb_int_ = sb->value();
        }

        // Smooth non-linear attack: inject pending wheel impulse into velocity over ~28ms
        if (std::abs(pend_impulse_) > 0.05) {
            const double inject_alpha = 1.0 - std::pow(0.00004, dt);
            const double step_imp = pend_impulse_ * inject_alpha;
            pend_impulse_ -= step_imp;
            vel_y_ += step_imp * 7.8;
        } else {
            pend_impulse_ = 0.0;
        }

        // Non-linear power-law drag: low friction at high velocity (kinetic glide) + quartic ease-out tail at low velocity
        const double spd = std::abs(vel_y_);
        const double non_linear_drag = 3.4 + 5.6 * std::pow(1.0 / (1.0 + spd / 220.0), 0.72);
        pos_y_ += vel_y_ * dt;
        vel_y_ *= std::exp(-non_linear_drag * dt);

        const double min_y = double(sb->minimum());
        const double max_y = double(sb->maximum());
        if (pos_y_ <= min_y) {
            pos_y_ = min_y;
            if (vel_y_ < 0.0) vel_y_ = 0.0;
            if (pend_impulse_ < 0.0) pend_impulse_ = 0.0;
        } else if (pos_y_ >= max_y) {
            pos_y_ = max_y;
            if (vel_y_ > 0.0) vel_y_ = 0.0;
            if (pend_impulse_ > 0.0) pend_impulse_ = 0.0;
        }

        const int sb_int = static_cast<int>(std::round(pos_y_));
        const double subpixel = pos_y_ - double(sb_int);
        last_sb_int_ = sb_int;
        list_->setProperty("smooth_scroll_y", pos_y_);
        list_->setProperty("last_sb_int", sb_int);
        list_->setProperty("subpixel_y", subpixel);
        if (sb->value() != sb_int) {
            sb->setValue(sb_int);
        }
        if (is_lyric_ && list_->viewport()) {
            list_->viewport()->update();
        }

        if (std::abs(vel_y_) < 4.0 && std::abs(pend_impulse_) < 0.3) {
            vel_y_ = 0.0;
            pend_impulse_ = 0.0;
            list_->setProperty("subpixel_y", 0.0);
            if (is_lyric_ && list_->viewport()) list_->viewport()->update();
            timer_->stop();
        }
    }
};

void tick_lyric_list(QListWidget* list, double dt_sec, bool& still_moving) {
    if (!list || list->count() == 0) return;
    const int active_row = list->property("active_row").isValid() ? list->property("active_row").toInt() : -1;
    if (active_row < 0 || active_row >= list->count()) return;

    double smooth_row = list->property("smooth_row").isValid()
        ? list->property("smooth_row").toDouble()
        : double(active_row);
    double row_vel = list->property("row_vel").toDouble();
    const double row_diff = double(active_row) - smooth_row;
    if (reduce_motion_on || std::abs(row_diff) > 5.0) {
        smooth_row = double(active_row);
        row_vel = 0.0;
    } else if (std::abs(row_diff) > 0.002 || std::abs(row_vel) > 0.015) {
        // Critically damped second-order non-linear spring-damper for active row scale/opacity
        const double omega_r = 14.0;
        const double zeta_r = 1.06;
        const double accel_r = omega_r * omega_r * row_diff - 2.0 * zeta_r * omega_r * row_vel;
        row_vel += accel_r * dt_sec;
        smooth_row += row_vel * dt_sec;
        still_moving = true;
    } else {
        smooth_row = double(active_row);
        row_vel = 0.0;
    }
    list->setProperty("smooth_row", smooth_row);
    list->setProperty("row_vel", row_vel);

    const qint64 hold_until = list->property("user_wheel_until_ms").toLongLong();
    const bool user_wheeling = (hold_until > QDateTime::currentMSecsSinceEpoch());

    if (!user_wheeling && list->isVisible() && list->verticalScrollBar()) {
        auto* sb = list->verticalScrollBar();
        auto* item = list->item(active_row);
        const QRect rect = list->visualItemRect(item);
        if (rect.isValid()) {
            const int vh = list->viewport() ? list->viewport()->height() : list->height();
            const double item_center_doc = double(sb->value()) + double(rect.top()) + double(rect.height()) * 0.5;
            const double target_y = std::clamp(item_center_doc - double(vh) * 0.5, double(sb->minimum()), double(sb->maximum()));

            double cur_y = list->property("smooth_scroll_y").isValid()
                ? list->property("smooth_scroll_y").toDouble()
                : double(sb->value());
            double vel_y = list->property("scroll_vel_y").toDouble();
            const int last_set_int = list->property("last_sb_int").isValid()
                ? list->property("last_sb_int").toInt()
                : sb->value();
            if (std::abs(sb->value() - last_set_int) > 3) {
                cur_y = double(sb->value());
                vel_y = 0.0;
            }

            const double y_diff = target_y - cur_y;
            if (reduce_motion_on) {
                cur_y = target_y;
                vel_y = 0.0;
            } else if (std::abs(y_diff) > 0.12 || std::abs(vel_y) > 1.2) {
                // Non-linear inertial spring-damper: smooth ease-in acceleration from rest + quintic-like ease-out deceleration tail
                const double dist = std::abs(y_diff);
                const double soft_diff = y_diff / (1.0 + dist / 460.0 * 0.32);
                const double omega_y = 10.6 + 2.2 / (1.0 + dist / 90.0);
                const double zeta_y = 1.08;
                const double accel_y = omega_y * omega_y * soft_diff - 2.0 * zeta_y * omega_y * vel_y;
                vel_y += accel_y * dt_sec;
                cur_y += vel_y * dt_sec;
                if ((y_diff > 0.0 && cur_y > target_y) || (y_diff < 0.0 && cur_y < target_y)) {
                    cur_y = target_y;
                    vel_y = 0.0;
                }
                still_moving = true;
            } else {
                cur_y = target_y;
                vel_y = 0.0;
            }

            const int sb_int = static_cast<int>(std::round(cur_y));
            const double subpixel = cur_y - double(sb_int);
            list->setProperty("smooth_scroll_y", cur_y);
            list->setProperty("scroll_vel_y", vel_y);
            list->setProperty("last_sb_int", sb_int);
            list->setProperty("subpixel_y", subpixel);
            if (sb->value() != sb_int) {
                sb->setValue(sb_int);
            }
        }
    }
    if (list->viewport()) list->viewport()->update();
}

void smooth_scroll_lyrics(int active_row) {
    if (active_row < 0) return;
    if (!lyric_anim_timer && window) {
        lyric_anim_timer = new QTimer(window);
        lyric_anim_timer->setTimerType(Qt::PreciseTimer);
        sync_timer_to_screen_refresh(lyric_anim_timer, window);
        QObject::connect(lyric_anim_timer, &QTimer::timeout, [] {
            sync_timer_to_screen_refresh(lyric_anim_timer, window);
            const double dt_sec = lyric_dt_timer.isValid()
                ? std::clamp(double(lyric_dt_timer.nsecsElapsed()) * 1e-9, 0.0005, 0.050)
                : (1.0 / current_screen_refresh_hz(window));
            lyric_dt_timer.restart();
            bool still_moving = false;
            tick_lyric_list(lyric_list, dt_sec, still_moving);
            tick_lyric_list(fullscreen_lyric_list, dt_sec, still_moving);
            if (!still_moving && lyric_anim_timer) {
                lyric_anim_timer->stop();
            }
        });
    }
    sync_timer_to_screen_refresh(lyric_anim_timer, window);
    lyric_dt_timer.restart();
    if (lyric_anim_timer && !lyric_anim_timer->isActive()) {
        lyric_anim_timer->start();
    }
}

void sync_active_lyric_from_pos(qint64 pos_ms) {
    if (!lyric_list || lyric_list->count() == 0) return;
    int active_row = -1;
    for (int i = 0; i < lyric_list->count(); ++i) {
        bool ok = false;
        const qint64 t = lyric_list->item(i)->data(Qt::UserRole).toLongLong(&ok);
        if (ok && t >= 0 && t <= pos_ms) {
            active_row = i;
        }
    }
    const int stored_row = lyric_list->property("active_row").isValid() ? lyric_list->property("active_row").toInt() : -2;
    if (active_row != stored_row) {
        lyric_list->setProperty("active_row", active_row);
        if (stored_row >= 0 && stored_row < lyric_list->count()) {
            lyric_list->item(stored_row)->setData(mineral_ui::RoleCurrent, false);
        }
        QString active_line;
        if (active_row >= 0 && active_row < lyric_list->count()) {
            auto* it = lyric_list->item(active_row);
            it->setData(mineral_ui::RoleCurrent, true);
            const QString lt = it->data(mineral_ui::RoleTitle).toString();
            const QString ls = it->data(mineral_ui::RoleSubtitle).toString();
            active_line = ls.isEmpty() ? lt : (lt + "  ·  " + ls);
        }
        if (fullscreen_lyric_list) {
            fullscreen_lyric_list->setProperty("active_row", active_row);
            if (stored_row >= 0 && stored_row < fullscreen_lyric_list->count()) {
                fullscreen_lyric_list->item(stored_row)->setData(mineral_ui::RoleCurrent, false);
            }
            if (active_row >= 0 && active_row < fullscreen_lyric_list->count()) {
                fullscreen_lyric_list->item(active_row)->setData(mineral_ui::RoleCurrent, true);
            }
        }
        if (now_artist && (!hero_banner || !hero_banner->immersive_mode)) {
            if (!active_line.isEmpty()) {
                now_artist->setText(base_hero_desc.isEmpty() ? active_line : (base_hero_desc + "\n" + active_line));
            } else if (!base_hero_desc.isEmpty()) {
                now_artist->setText(base_hero_desc);
            }
        }
        smooth_scroll_lyrics(active_row);
    }
}

void refresh_lyric_item_sizes() {
    auto update_list = [](QListWidget* list, bool hero) {
        if (!list) return;
        const int base_px = hero ? mineral_ui::fullscreen_lyric_font_px : mineral_ui::normal_lyric_font_px;
        const int sub_px = std::max(11, base_px - (hero ? 5 : 3));
        for (int i = 0; i < list->count(); ++i) {
            auto* it = list->item(i);
            const QString sub = it->data(mineral_ui::RoleSubtitle).toString();
            const int h = sub.isEmpty() ? (base_px + (hero ? 26 : 22)) : (base_px + sub_px + (hero ? 32 : 26));
            it->setSizeHint(QSize(0, h));
        }
        list->doItemsLayout();
        if (list->viewport()) list->viewport()->update();
    };
    update_list(lyric_list, false);
    update_list(fullscreen_lyric_list, true);
    const int active_row = lyric_list ? lyric_list->property("active_row").toInt() : -1;
    if (active_row >= 0) smooth_scroll_lyrics(active_row);
}

void adjust_lyric_font_size(int delta) {
    QSettings qs("mineral", "mineral-gui-qt");
    if (hero_banner && hero_banner->immersive_mode) {
        mineral_ui::fullscreen_lyric_font_px = std::clamp(mineral_ui::fullscreen_lyric_font_px + delta, 14, 38);
        qs.setValue("fullscreen_lyric_font_px", mineral_ui::fullscreen_lyric_font_px);
    } else {
        mineral_ui::normal_lyric_font_px = std::clamp(mineral_ui::normal_lyric_font_px + delta, 12, 26);
        mineral_ui::fullscreen_lyric_font_px = std::clamp(mineral_ui::fullscreen_lyric_font_px + delta, 14, 38);
        qs.setValue("normal_lyric_font_px", mineral_ui::normal_lyric_font_px);
        qs.setValue("fullscreen_lyric_font_px", mineral_ui::fullscreen_lyric_font_px);
    }
    refresh_lyric_item_sizes();
}

void refresh_md3_icons() {
    if (play_button) {
        play_button->setIcon(mineral_ui::md3_icon(
            last_playing_state ? mineral_ui::M3Icon::Pause : mineral_ui::M3Icon::Play, QColor("#FFFFFF"), 20));
        play_button->setIconSize(QSize(20, 20));
    }
    if (prev_btn_ref) prev_btn_ref->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Prev, QColor("#FFFFFF"), 15));
    if (next_btn_ref) next_btn_ref->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Next, QColor("#FFFFFF"), 15));
    if (mode_cycle_btn) {
        mode_cycle_btn->setText(QString());
        mode_cycle_btn->setIcon(mineral_ui::md3_icon(play_mode_icon(last_play_mode), QColor("#FFFFFF"), 16));
        mode_cycle_btn->setIconSize(QSize(16, 16));
    }
    if (love_btn) {
        love_btn->setText(QString());
        love_btn->setIcon(mineral_ui::md3_icon(
            last_loved_state ? mineral_ui::M3Icon::HeartFilled : mineral_ui::M3Icon::HeartOutline,
            QColor("#FFFFFF"), 16));
        love_btn->setIconSize(QSize(16, 16));
    }
    if (queue_btn_ref) queue_btn_ref->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Queue, QColor("#FFFFFF"), 15));
    if (mini_btn_ref) mini_btn_ref->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::MiniPlayer, QColor("#FFFFFF"), 15));
    if (immersive_btn_ref) immersive_btn_ref->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Fullscreen, QColor("#FFFFFF"), 15));
    if (search_icon_btn) {
        search_icon_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Search, QColor("#FFFFFF"), 15));
        search_icon_btn->setIconSize(QSize(15, 15));
    }
    if (search_icon_label) {
        search_icon_label->setPixmap(mineral_ui::md3_icon(mineral_ui::M3Icon::Search, QColor("#FFFFFF"), 16).pixmap(16, 16));
    }
    if (tray_icon) {
        tray_icon->setIcon(make_tray_icon());
    }
}

void apply_m3_theme() {
    if (!window) return;
    const QString st = shell_style(touch_on);
    window->setStyleSheet(st);
    if (mode_dialog) mode_dialog->setStyleSheet(st);
    if (palette_dialog) palette_dialog->setStyleSheet(st);
    if (help_dialog) help_dialog->setStyleSheet(st);
    refresh_md3_icons();
    if (sidebar) sidebar->update();
    if (rows && rows->viewport()) rows->viewport()->update();
    if (lyric_list && lyric_list->viewport()) lyric_list->viewport()->update();
    if (fullscreen_lyric_list && fullscreen_lyric_list->viewport()) fullscreen_lyric_list->viewport()->update();
    if (viz_widget) viz_widget->update();
}

QDialog* make_card(const QString& title, const QString& placeholder, QLineEdit** search_out, QListWidget** list_out, bool with_combo_save = false) {
    auto* dialog = new QDialog(window);
    dialog->setWindowTitle(title);
    dialog->setModal(false);
    dialog->setStyleSheet(shell_style());
    dialog->resize(580, 540);
    auto* layout = new QVBoxLayout(dialog);
    layout->setContentsMargins(20, 18, 20, 18);
    layout->setSpacing(12);
    auto* label = new QLabel(title);
    label->setObjectName("heading");
    layout->addWidget(label);
    auto* hint = new QLabel("Applies immediately while sharing the same play queue and playback state.");
    hint->setObjectName("muted");
    layout->addWidget(hint);
    auto* search = new QLineEdit();
    search->setPlaceholderText(placeholder);
    search->setClearButtonEnabled(true);
    layout->addWidget(search);
    auto* list = new QListWidget();
    layout->addWidget(list, 1);
    if (with_combo_save) {
        auto* combo_row = new QHBoxLayout();
        combo_name_box = new QLineEdit();
        combo_name_box->setPlaceholderText("Enter name to save current layout preset…");
        auto* save_btn = new QPushButton("Save Preset");
        save_btn->setObjectName("ghost");
        combo_row->addWidget(combo_name_box, 1);
        combo_row->addWidget(save_btn);
        layout->addLayout(combo_row);
        QObject::connect(save_btn, &QPushButton::clicked, [] {
            const QString name = combo_name_box ? escape_json(combo_name_box->text()) : QString();
            push_action(QString("{\"action\":\"save_combo\",\"name\":\"%1\"}").arg(name));
            if (combo_name_box) combo_name_box->clear();
        });
    }
    *search_out = search;
    *list_out = list;
    return dialog;
}

QString find_mineral_binary() {
#ifdef Q_OS_WIN
    const QString exe_name = QStringLiteral("mineral.exe");
    const QString bridge_name = QStringLiteral("mineral-gui-bridge.exe");
#else
    const QString exe_name = QStringLiteral("mineral");
    const QString bridge_name = QStringLiteral("mineral-gui-bridge");
#endif
    const QString bridge_beside = QDir(QCoreApplication::applicationDirPath()).filePath(bridge_name);
    if (QFileInfo::exists(bridge_beside)) {
        return bridge_beside;
    }
    const QString bridge_in_path = QStandardPaths::findExecutable(QStringLiteral("mineral-gui-bridge"));
    if (!bridge_in_path.isEmpty()) {
        return bridge_in_path;
    }
    const QString beside_app = QDir(QCoreApplication::applicationDirPath()).filePath(exe_name);
    if (QFileInfo::exists(beside_app)) {
        return beside_app;
    }
    const QString in_path = QStandardPaths::findExecutable(QStringLiteral("mineral"));
    if (!in_path.isEmpty()) {
        return in_path;
    }
    const QString cargo_bin = QDir(QDir::homePath()).filePath(QStringLiteral(".cargo/bin/") + exe_name);
    if (QFileInfo::exists(cargo_bin)) {
        return cargo_bin;
    }
    return QStringLiteral("mineral");
}

QPixmap render_dense1x2_qr(const QStringList& qr_lines) {
    int max_cols = 0;
    for (const QString& line : qr_lines) {
        max_cols = std::max(max_cols, static_cast<int>(line.size()));
    }
    const int char_rows = static_cast<int>(qr_lines.size());
    if (max_cols < 16 || char_rows < 8) return QPixmap();
    const int px_w = max_cols;
    const int px_h = char_rows * 2;
    std::vector<int> grid(static_cast<size_t>(px_w * px_h), 0);
    for (int r = 0; r < char_rows; ++r) {
        const QString& line = qr_lines[r];
        for (int c = 0; c < line.size(); ++c) {
            const ushort u = line[c].unicode();
            int top_bit = 0;
            int bot_bit = 0;
            if (u == 0x2588) { // '█'
                top_bit = 1;
                bot_bit = 1;
            } else if (u == 0x2580) { // '▀'
                top_bit = 1;
                bot_bit = 0;
            } else if (u == 0x2584) { // '▄'
                top_bit = 0;
                bot_bit = 1;
            }
            grid[static_cast<size_t>((2 * r) * px_w + c)] = top_bit;
            grid[static_cast<size_t>((2 * r + 1) * px_w + c)] = bot_bit;
        }
    }
    const int bg_bit = grid[0];
    QImage img(px_w, px_h, QImage::Format_RGB32);
    const QRgb white = qRgb(255, 255, 255);
    const QRgb dark = qRgb(26, 22, 21);
    for (int y = 0; y < px_h; ++y) {
        for (int x = 0; x < px_w; ++x) {
            const int b = grid[static_cast<size_t>(y * px_w + x)];
            img.setPixel(x, y, (b == bg_bit) ? white : dark);
        }
    }
    return QPixmap::fromImage(img.scaled(264, 264, Qt::KeepAspectRatio, Qt::FastTransformation));
}

void open_qr_login_dialog(const QString& channel_id) {
    const bool is_bili = (channel_id == QStringLiteral("bilibili"));
    const QString brand = is_bili ? QStringLiteral("Bilibili") : QStringLiteral("NetEase Cloud Music");

    auto* dlg = new QDialog(window);
    dlg->setWindowTitle(brand + QStringLiteral(" — QR Code Login"));
    dlg->setModal(true);
    dlg->setStyleSheet(shell_style(touch_on));
    dlg->resize(430, 500);

    auto* v = new QVBoxLayout(dlg);
    v->setContentsMargins(24, 20, 24, 20);
    v->setSpacing(12);

    auto* title_lbl = new QLabel(brand + QStringLiteral(" QR Code Login"));
    title_lbl->setObjectName("heading");
    title_lbl->setAlignment(Qt::AlignCenter);
    v->addWidget(title_lbl);

    auto* sub_lbl = new QLabel(QStringLiteral("Open the ") + brand + QStringLiteral(" mobile app and scan the QR code below to sign in:"));
    sub_lbl->setObjectName("muted");
    sub_lbl->setWordWrap(true);
    sub_lbl->setAlignment(Qt::AlignCenter);
    v->addWidget(sub_lbl);

    auto* qr_card = new QFrame();
    qr_card->setFixedSize(292, 292);
    qr_card->setStyleSheet(
        "QFrame { background: #FFFFFF; border: 1px solid #E4DCD6; border-radius: 18px; }"
    );
    auto* qr_card_layout = new QVBoxLayout(qr_card);
    qr_card_layout->setContentsMargins(14, 14, 14, 14);
    auto* qr_lbl = new QLabel(QStringLiteral("Generating QR Code…"));
    qr_lbl->setFixedSize(264, 264);
    qr_lbl->setAlignment(Qt::AlignCenter);
    qr_lbl->setStyleSheet("QLabel { color: #7C7370; font-size: 13px; border: none; background: transparent; }");
    qr_card_layout->addWidget(qr_lbl, 0, Qt::AlignCenter);
    v->addWidget(qr_card, 0, Qt::AlignCenter);

    auto* status_lbl = new QLabel(QStringLiteral("Requesting login QR code from ") + brand + QStringLiteral("…"));
    status_lbl->setObjectName("muted");
    status_lbl->setWordWrap(true);
    status_lbl->setAlignment(Qt::AlignCenter);
    v->addWidget(status_lbl);

    v->addStretch(1);

    auto* btn_row = new QHBoxLayout();
    auto* refresh_btn = new QPushButton(QStringLiteral("Refresh QR Code"));
    refresh_btn->setObjectName("ghost");
    refresh_btn->setCursor(Qt::PointingHandCursor);
    auto* close_btn = new QPushButton(QStringLiteral("Close"));
    close_btn->setObjectName("chip");
    close_btn->setCursor(Qt::PointingHandCursor);
    btn_row->addStretch(1);
    btn_row->addWidget(refresh_btn);
    btn_row->addWidget(close_btn);
    btn_row->addStretch(1);
    v->addLayout(btn_row);

    auto* proc = new QProcess(dlg);
    auto* stdout_buf = new QString();
    auto* qr_rendered = new bool(false);

    auto start_login_proc = [proc, channel_id, qr_lbl, status_lbl, brand, stdout_buf, qr_rendered]() {
        if (proc->state() != QProcess::NotRunning) {
            proc->kill();
            proc->waitForFinished(300);
        }
        stdout_buf->clear();
        *qr_rendered = false;
        qr_lbl->setPixmap(QPixmap());
        qr_lbl->setText(QStringLiteral("Generating QR Code…"));
        status_lbl->setText(QStringLiteral("Requesting login QR code from ") + brand + QStringLiteral("…"));
        proc->start(find_mineral_binary(), {QStringLiteral("channel"), channel_id, QStringLiteral("login")});
    };

    QObject::connect(proc, &QProcess::readyReadStandardOutput, dlg, [proc, qr_lbl, status_lbl, stdout_buf, qr_rendered]() {
        *stdout_buf += QString::fromUtf8(proc->readAllStandardOutput());
        const QStringList lines = stdout_buf->split('\n');
        QStringList qr_lines;
        for (QString line : lines) {
            if (line.endsWith('\r')) line.chop(1);
            if (line.startsWith(QStringLiteral("Logged in."))) {
                status_lbl->setText(line);
                continue;
            }
            if (line.size() < 16) continue;
            bool is_qr_row = true;
            for (const QChar ch : line) {
                const ushort u = ch.unicode();
                if (u != 0x2588 && u != 0x2580 && u != 0x2584 && u != 0x0020) {
                    is_qr_row = false;
                    break;
                }
            }
            if (is_qr_row) {
                qr_lines.push_back(line);
            }
        }
        if (!*qr_rendered && qr_lines.size() >= 14) {
            const QPixmap pm = render_dense1x2_qr(qr_lines);
            if (!pm.isNull()) {
                *qr_rendered = true;
                qr_lbl->setText(QString());
                qr_lbl->setPixmap(pm);
                status_lbl->setText(QStringLiteral("Scan the QR code with your phone and confirm login."));
            }
        }
    });

    QObject::connect(proc, &QProcess::readyReadStandardError, dlg, [proc, status_lbl, brand]() {
        const QString err_text = QString::fromUtf8(proc->readAllStandardError());
        const QStringList lines = err_text.split('\n', Qt::SkipEmptyParts);
        for (QString line : lines) {
            line = line.trimmed();
            if (line.contains(QStringLiteral("waiting for phone confirmation"), Qt::CaseInsensitive)) {
                status_lbl->setText(QStringLiteral("Scanned! Please tap 'Confirm Login' on your phone…"));
            } else if (line.contains(QStringLiteral("waiting for scan"), Qt::CaseInsensitive)
                       || line.contains(QStringLiteral("scan and confirm"), Qt::CaseInsensitive)) {
                status_lbl->setText(QStringLiteral("Waiting for ") + brand + QStringLiteral(" mobile app to scan the QR code…"));
            } else if (!line.isEmpty()) {
                status_lbl->setText(line);
            }
        }
    });

    QObject::connect(proc,
                     QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
                     dlg,
                     [dlg, status_lbl, brand](int exit_code, QProcess::ExitStatus status) {
                         if (status == QProcess::NormalExit && exit_code == 0) {
                             status_lbl->setText(brand + QStringLiteral(" login successful! Reloading daemon to sync your account…"));
                             push_action("{\"action\":\"restart_daemon\"}");
                             QTimer::singleShot(900, dlg, &QDialog::accept);
                         } else if (status == QProcess::NormalExit) {
                             status_lbl->setText(QStringLiteral("Login session ended or expired (code ")
                                                 + QString::number(exit_code)
                                                 + QStringLiteral("). Click 'Refresh QR Code' to try again."));
                         }
                     });

    QObject::connect(refresh_btn, &QPushButton::clicked, dlg, start_login_proc);
    QObject::connect(close_btn, &QPushButton::clicked, dlg, &QDialog::reject);

    start_login_proc();
    dlg->exec();
    if (proc->state() != QProcess::NotRunning) {
        proc->kill();
        proc->waitForFinished(300);
    }
    delete stdout_buf;
    delete qr_rendered;
    dlg->deleteLater();
}
void open_am_token_dialog() {
    auto* dlg = new QDialog(window);
    dlg->setWindowTitle(QStringLiteral("Apple Music — Token Login"));
    dlg->setModal(true);
    dlg->setStyleSheet(shell_style(touch_on));
    dlg->resize(520, 260);

    auto* v = new QVBoxLayout(dlg);
    v->setContentsMargins(24, 20, 24, 20);
    v->setSpacing(12);

    auto* title_lbl = new QLabel(QStringLiteral("Apple Music Token Login"));
    title_lbl->setObjectName("heading");
    title_lbl->setAlignment(Qt::AlignCenter);
    v->addWidget(title_lbl);

    auto* sub_lbl = new QLabel(QStringLiteral("Sign in at music.apple.com, copy the media-user-token cookie value from browser DevTools, and paste it below:"));
    sub_lbl->setObjectName("muted");
    sub_lbl->setWordWrap(true);
    sub_lbl->setAlignment(Qt::AlignCenter);
    v->addWidget(sub_lbl);

    auto* token_edit = new QLineEdit();
    token_edit->setPlaceholderText(QStringLiteral("Paste media-user-token cookie here…"));
    token_edit->setEchoMode(QLineEdit::PasswordEchoOnEdit);
    token_edit->setClearButtonEnabled(true);
    token_edit->setMinimumHeight(36);
    v->addWidget(token_edit);

    auto* status_lbl = new QLabel();
    status_lbl->setObjectName("muted");
    status_lbl->setWordWrap(true);
    status_lbl->setAlignment(Qt::AlignCenter);
    v->addWidget(status_lbl);

    v->addStretch(1);

    auto* btn_row = new QHBoxLayout();
    auto* cancel_btn = new QPushButton(QStringLiteral("Cancel"));
    cancel_btn->setObjectName("ghost");
    cancel_btn->setCursor(Qt::PointingHandCursor);
    auto* login_btn = new QPushButton(QStringLiteral("Sign In"));
    login_btn->setObjectName("chip");
    login_btn->setCursor(Qt::PointingHandCursor);
    btn_row->addStretch(1);
    btn_row->addWidget(cancel_btn);
    btn_row->addWidget(login_btn);
    btn_row->addStretch(1);
    v->addLayout(btn_row);

    auto* proc = new QProcess(dlg);
    auto* err_buf = new QString();
    auto* out_buf = new QString();
    auto* pending_token = new QString();

    QObject::connect(proc, &QProcess::readyReadStandardOutput, dlg, [proc, out_buf]() {
        *out_buf += QString::fromUtf8(proc->readAllStandardOutput());
    });

    QObject::connect(proc, &QProcess::readyReadStandardError, dlg, [proc, err_buf]() {
        *err_buf += QString::fromUtf8(proc->readAllStandardError());
    });

    QObject::connect(proc, &QProcess::started, dlg, [proc, pending_token]() {
        proc->write(pending_token->toUtf8() + "\n");
        proc->closeWriteChannel();
    });

    QObject::connect(proc, &QProcess::errorOccurred, dlg, [token_edit, login_btn, status_lbl](QProcess::ProcessError err) {
        token_edit->setEnabled(true);
        login_btn->setEnabled(true);
        if (err == QProcess::FailedToStart) {
            status_lbl->setText(QStringLiteral("Failed to start mineral CLI process."));
        }
    });

    auto start_token_login = [proc, token_edit, status_lbl, login_btn, err_buf, out_buf, pending_token]() {
        QString raw_token = token_edit->text().trimmed();
        if (raw_token.startsWith(QStringLiteral("media-user-token="), Qt::CaseInsensitive)) {
            raw_token = raw_token.mid(17).trimmed();
        }
        if (raw_token.endsWith(';')) {
            raw_token.chop(1);
            raw_token = raw_token.trimmed();
        }
        if (raw_token.isEmpty()) {
            status_lbl->setText(QStringLiteral("Token cannot be empty. Please enter your media-user-token."));
            return;
        }
        if (proc->state() != QProcess::NotRunning) {
            proc->kill();
            proc->waitForFinished(300);
        }
        err_buf->clear();
        out_buf->clear();
        *pending_token = raw_token;
        token_edit->setEnabled(false);
        login_btn->setEnabled(false);
        status_lbl->setText(QStringLiteral("Validating Apple Music account, please wait…"));

        proc->start(find_mineral_binary(), {QStringLiteral("channel"), QStringLiteral("am"), QStringLiteral("login")});
    };

    QObject::connect(proc,
                     QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
                     dlg,
                     [dlg, status_lbl, token_edit, login_btn, err_buf, out_buf](int exit_code, QProcess::ExitStatus status) {
                         token_edit->setEnabled(true);
                         login_btn->setEnabled(true);
                         if (status == QProcess::NormalExit && exit_code == 0) {
                             status_lbl->setText(QStringLiteral("Apple Music login successful! Reloading daemon to sync your account…"));
                             push_action("{\"action\":\"restart_daemon\"}");
                             QTimer::singleShot(900, dlg, &QDialog::accept);
                         } else {
                             QString msg = err_buf->trimmed();
                             if (msg.isEmpty()) msg = out_buf->trimmed();
                             if (msg.isEmpty()) {
                                 msg = QStringLiteral("Login failed (exit code %1).").arg(exit_code);
                             }
                             status_lbl->setText(msg);
                         }
                     });

    QObject::connect(login_btn, &QPushButton::clicked, dlg, start_token_login);
    QObject::connect(token_edit, &QLineEdit::returnPressed, dlg, start_token_login);
    QObject::connect(cancel_btn, &QPushButton::clicked, dlg, &QDialog::reject);

    dlg->exec();
    if (proc->state() != QProcess::NotRunning) {
        proc->kill();
        proc->waitForFinished(300);
    }
    delete err_buf;
    delete out_buf;
    delete pending_token;
    dlg->deleteLater();
}

}  // namespace

extern "C" void qt_init() {
    static int argc = 1;
    static char name[] = "mineral-gui-qt";
    static char* argv[] = {name, nullptr};
    app = new QApplication(argc, argv);
    app->setQuitOnLastWindowClosed(false);
    const QIcon app_icon = make_tray_icon();
    app->setWindowIcon(app_icon);
    {
        QSettings qs("mineral", "mineral-gui-qt");
        first_run_tip_shown = qs.value("first_run_tip_shown", false).toBool();
        mineral_ui::fullscreen_lyric_font_px = std::clamp(qs.value("fullscreen_lyric_font_px", 20).toInt(), 14, 38);
        mineral_ui::normal_lyric_font_px = std::clamp(qs.value("normal_lyric_font_px", 15).toInt(), 12, 26);
    }
    mineral_ui::on_m3_theme_changed = [] { apply_m3_theme(); };
    app->setStyleSheet(shell_style());
    app->setWindowIcon(make_tray_icon());
    window = new Window();
    window->setWindowTitle("Mineral");
    window->setWindowIcon(app_icon);
    window->setWindowIcon(make_tray_icon());
    window->resize(1380, 860);

    auto* root = new QWidget();
    root->setObjectName("root");
    auto* root_h_layout = new QHBoxLayout(root);
    root_h_layout->setContentsMargins(0, 0, 0, 0);
    root_h_layout->setSpacing(0);

    // ── ZONE 1: Left Warm Coral Navigation Sidebar (208px expanded / 64px collapsed icon-only) ──
    sidebar = new QWidget();
    sidebar->setObjectName("navRail");
    sidebar->setFixedWidth(208);
    auto* rail_layout = new QVBoxLayout(sidebar);
    rail_layout->setContentsMargins(18, 22, 18, 18);
    rail_layout->setSpacing(6);
    rail_layout_ref = rail_layout;
    side_section_labels.clear();
    side_nav_buttons.clear();

    // Top row: Search Pill + Sidebar Collapse/Expand Toggle Button
    auto* side_top_row = new QHBoxLayout();
    side_top_row->setContentsMargins(0, 0, 0, 0);
    side_top_row->setSpacing(6);
    side_top_row_ref = side_top_row;

    auto* search_bar_frame = new QFrame();
    search_bar_frame->setObjectName("searchBar");
    search_bar_frame->setFixedHeight(36);
    search_bar_frame->setMinimumWidth(0);
    search_bar_frame_ref = search_bar_frame;
    auto* sb_layout = new QHBoxLayout(search_bar_frame);
    sb_layout->setContentsMargins(14, 0, 8, 0);
    sb_layout->setSpacing(4);

    search_box = new QLineEdit();
    search_box->setObjectName("searchInput");
    search_box->setPlaceholderText("Search");
    search_box->setClearButtonEnabled(true);
    search_box->setMinimumWidth(0);
    sb_layout->addWidget(search_box, 1, Qt::AlignVCenter);

    search_icon_btn = new QPushButton();
    search_icon_btn->setObjectName("searchBtn");
    search_icon_btn->setFixedSize(24, 24);
    search_icon_btn->setIconSize(QSize(15, 15));
    search_icon_btn->setCursor(Qt::PointingHandCursor);
    search_icon_btn->setToolTip("Click to Search");
    search_icon_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Search, QColor("#FFFFFF"), 15));
    sb_layout->addWidget(search_icon_btn, 0, Qt::AlignVCenter);

    sidebar_toggle_btn = new QPushButton();
    sidebar_toggle_btn->setObjectName("iconBtn");
    sidebar_toggle_btn->setCursor(Qt::PointingHandCursor);
    sidebar_toggle_btn->setToolTip("Collapse / Expand Sidebar");
    sidebar_toggle_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::SidebarCollapse, QColor("#FFFFFF"), 16));

    side_top_row->addWidget(search_bar_frame, 1);
    side_top_row->addWidget(sidebar_toggle_btn, 0, Qt::AlignCenter);
    rail_layout->addLayout(side_top_row);
    rail_layout->addSpacing(16);

    nav_group = new QButtonGroup(sidebar);
    nav_group->setExclusive(true);

    auto add_side_section = [&](const char* title) {
        auto* lbl = new QLabel(title);
        lbl->setObjectName("sideSection");
        lbl->setMinimumWidth(0);
        side_section_labels.push_back(lbl);
        rail_layout->addWidget(lbl);
        rail_layout->addSpacing(2);
    };

    auto add_nav_item = [&](const char* id, const char* title, mineral_ui::M3Icon ic, bool checked = false) {
        auto* button = new mineral_ui::M3NavRailButton(ic, title, sidebar);
        button->setProperty("nav", id);
        if (checked) button->setChecked(true);
        nav_group->addButton(button);
        side_nav_buttons.push_back(button);
        rail_layout->addWidget(button);
        QObject::connect(button, &QPushButton::clicked, [id = QString(id)] {
            now_playing_tab_active = false;
            play_queue_tab_active = false;
            if (lyrics_card) lyrics_card->setVisible(false);
            if (center_panel) center_panel->setVisible(true);
            if (bottom_bento_area) bottom_bento_area->setVisible(true);
            if (current_workspace == "covers") {
                push_action("{\"action\":\"scheme\",\"scheme\":\"classic\"}");
            }
            push_action(QString("{\"action\":\"nav\",\"nav\":\"%1\"}").arg(id));
        });
    };

    add_side_section("BROWSE MUSIC");
    add_nav_item("library", "Discover", mineral_ui::M3Icon::Library, true);
    add_nav_item("playlists", "Playlists", mineral_ui::M3Icon::Playlists, false);

    rail_layout->addSpacing(12);
    add_side_section("YOUR MUSIC");
    add_nav_item("loved", "Favourites", mineral_ui::M3Icon::HeartFilled, false);
    add_nav_item("downloads", "Downloads", mineral_ui::M3Icon::Downloads, false);

    rail_layout->addSpacing(12);
    add_side_section("PLAYER");
    auto* now_playing_nav_btn = new mineral_ui::M3NavRailButton(mineral_ui::M3Icon::Equalizer, "Now Playing", sidebar);
    now_playing_nav_btn->setProperty("nav", "now_playing");
    nav_group->addButton(now_playing_nav_btn);
    side_nav_buttons.push_back(now_playing_nav_btn);
    rail_layout->addWidget(now_playing_nav_btn);
    QObject::connect(now_playing_nav_btn, &QPushButton::clicked, [] {
        now_playing_tab_active = true;
        play_queue_tab_active = false;
        if (center_panel) center_panel->setVisible(false);
        if (bottom_bento_area) bottom_bento_area->setVisible(false);
        if (lyrics_card) lyrics_card->setVisible(true);
        if (lyric_font_dec_btn) lyric_font_dec_btn->setVisible(true);
        if (lyric_font_inc_btn) lyric_font_inc_btn->setVisible(true);
        if (lyric_list) {
            const int cur_r = lyric_list->property("active_row").toInt();
            if (cur_r >= 0) smooth_scroll_lyrics(cur_r);
        }
    });

    auto* side_queue_btn = new mineral_ui::M3NavRailButton(mineral_ui::M3Icon::Queue, "Play Queue", sidebar);
    side_queue_btn->setProperty("nav", "queue");
    nav_group->addButton(side_queue_btn);
    side_nav_buttons.push_back(side_queue_btn);
    rail_layout->addWidget(side_queue_btn);
    QObject::connect(side_queue_btn, &QPushButton::clicked, [] {
        now_playing_tab_active = false;
        play_queue_tab_active = true;
        if (lyrics_card) lyrics_card->setVisible(false);
        if (center_panel) center_panel->setVisible(true);
        if (bottom_bento_area) bottom_bento_area->setVisible(true);
        if (current_workspace == "covers") {
            push_action("{\"action\":\"scheme\",\"scheme\":\"classic\"}");
        }
        push_action("{\"action\":\"nav\",\"nav\":\"queue\"}");
    });

    add_nav_item("settings", "Settings", mineral_ui::M3Icon::Settings, false);

    rail_layout->addStretch();

    // Bottom subtle status & power-user menu inside Coral Sidebar
    auto* side_footer = new QHBoxLayout();
    side_footer->setContentsMargins(2, 0, 0, 0);
    side_footer->setSpacing(6);
    side_footer_ref = side_footer;

    daemon_btn = new QPushButton("● Online");
    daemon_btn->setObjectName("heroTag");
    daemon_btn->setMinimumWidth(0);
    daemon_btn->setCursor(Qt::PointingHandCursor);
    daemon_btn->setToolTip("Click to reconnect or launch playback daemon");
    QObject::connect(daemon_btn, &QPushButton::clicked, [] {
        push_action(daemon_connected ? "{\"action\":\"connect\"}" : "{\"action\":\"launch_daemon\"}");
    });
    daemon_label = new QLabel("● Online");
    daemon_label->setVisible(false);
    side_footer->addWidget(daemon_btn);
    side_footer->addStretch();

    auto* help_rail_btn = new QPushButton();
    help_rail_btn->setObjectName("iconBtn");
    help_rail_btn->setMinimumWidth(0);
    help_rail_btn->setToolTip("Keyboard Shortcuts (?)");
    help_rail_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Help, QColor("#FFFFFF"), 16));
    QObject::connect(help_rail_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"help\",\"open\":true}"); });
    help_rail_btn_ref = help_rail_btn;
    side_footer->addWidget(help_rail_btn);

    auto update_dyn_btn_label = [] {
        if (!dyn_color_btn) return;
        if (mineral_ui::dynamic_cover_color) {
            dyn_color_btn->setText("Dynamic Color");
        } else {
            const QString p = mineral_ui::current_m3_preset;
            if (p == "sage") dyn_color_btn->setText("Theme: Sage");
            else if (p == "lavender") dyn_color_btn->setText("Theme: Lavender");
            else if (p == "ocean") dyn_color_btn->setText("Theme: Ocean");
            else dyn_color_btn->setText("Theme: Coral");
        }
    };

    auto* more = new QPushButton();
    more->setObjectName("iconBtn");
    more->setToolTip("More Actions & Themes");
    more->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::MoreVert, QColor("#FFFFFF"), 16));
    auto* menu = new QMenu(more);
    menu->addAction("Command Palette (Ctrl+K)", [] { push_action("{\"action\":\"palette\"}"); });
    menu->addAction("Keyboard Shortcuts (?)", [] { push_action("{\"action\":\"help\",\"open\":true}"); });
    menu->addSeparator();
    menu->addAction("Theme: Dynamic Cover Color (Auto)", [update_dyn_btn_label] {
        mineral_ui::dynamic_cover_color = true;
        mineral_ui::current_m3_preset = "dynamic";
        if (cover_widget) cover_widget->applyDynamicPalette();
        update_dyn_btn_label();
        apply_m3_theme();
    });
    menu->addAction("Theme: Warm Coral", [update_dyn_btn_label] {
        mineral_ui::dynamic_cover_color = false;
        mineral_ui::current_m3_preset = "coral";
        mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("coral");
        if (cover_widget) cover_widget->applyDynamicPalette();
        update_dyn_btn_label();
        apply_m3_theme();
    });
    menu->addAction("Theme: Warm Sage", [update_dyn_btn_label] {
        mineral_ui::dynamic_cover_color = false;
        mineral_ui::current_m3_preset = "sage";
        mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("sage");
        if (cover_widget) cover_widget->applyDynamicPalette();
        update_dyn_btn_label();
        apply_m3_theme();
    });
    menu->addAction("Theme: Editorial Lavender", [update_dyn_btn_label] {
        mineral_ui::dynamic_cover_color = false;
        mineral_ui::current_m3_preset = "lavender";
        mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("lavender");
        if (cover_widget) cover_widget->applyDynamicPalette();
        update_dyn_btn_label();
        apply_m3_theme();
    });
    menu->addAction("Theme: Nordic Ocean", [update_dyn_btn_label] {
        mineral_ui::dynamic_cover_color = false;
        mineral_ui::current_m3_preset = "ocean";
        mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("ocean");
        if (cover_widget) cover_widget->applyDynamicPalette();
        update_dyn_btn_label();
        apply_m3_theme();
    });
    menu->addSeparator();
    menu->addAction("Toggle Mini Player", [] {
        push_action(was_mini ? "{\"action\":\"back\"}" : "{\"action\":\"scheme\",\"scheme\":\"mini\"}");
    });
    menu->addAction("Toggle Fullscreen Player (Z)", [] { push_action("{\"action\":\"toggle_immersive\"}"); });
    menu->addSeparator();
    menu->addAction("Login: Apple Music (Token)…", [] { open_am_token_dialog(); });
    menu->addAction("Login: NetEase Cloud Music (QR Code)…", [] { open_qr_login_dialog(QStringLiteral("netease")); });
    menu->addAction("Login: Bilibili (QR Code)…", [] { open_qr_login_dialog(QStringLiteral("bilibili")); });
    menu->addSeparator();
    menu->addAction("Reconnect Daemon", [] { push_action("{\"action\":\"connect\"}"); });
    menu->addAction("Launch Playback Daemon", [] { push_action("{\"action\":\"launch_daemon\"}"); });
    menu->addAction("Play All in View", [] { push_action("{\"action\":\"play_container\",\"key\":\"\"}"); });
    menu->addAction("Play Selected Next", [] { push_action("{\"action\":\"play_next\",\"key\":\"\"}"); });
    menu->addAction("Add Selected to Queue", [] { push_action("{\"action\":\"enqueue\",\"key\":\"\"}"); });
    menu->addAction("Toggle Favourite", [] { push_action("{\"action\":\"love\",\"key\":\"\"}"); });
    menu->addAction("Download Selected", [] { push_action("{\"action\":\"download\",\"key\":\"\"}"); });
    more->setMenu(menu);
    side_footer->addWidget(more);
    rail_layout->addLayout(side_footer);

    auto apply_sidebar_anim_frame = [](double raw_t) {
        const double t = std::clamp(raw_t, 0.0, 1.0);
        const double sf = std::clamp(search_focus_t, 0.0, 1.25);
        const double ff = std::clamp(filter_focus_t, 0.0, 1.25);
        const double base_w = 208.0 + 70.0 * sf * (1.0 - t);
        const int w = static_cast<int>(std::round(base_w + (64.0 - base_w) * t));
        if (sidebar) sidebar->setFixedWidth(w);

        const int mx = static_cast<int>(std::round(18.0 + (11.0 - 18.0) * t));
        if (rail_layout_ref) {
            rail_layout_ref->setContentsMargins(mx, 22, mx, 18);
        }

        if (sidebar_toggle_btn) {
            if (t <= 0.005 && sf > 0.01) {
                const int tb_w = static_cast<int>(std::round(30.0 * std::clamp(1.0 - sf * 1.15, 0.0, 1.0)));
                const bool show_tb = tb_w > 4;
                sidebar_toggle_btn->setVisible(show_tb);
                sidebar_toggle_btn->setMaximumWidth(show_tb ? tb_w : 0);
            } else {
                sidebar_toggle_btn->setVisible(true);
                sidebar_toggle_btn->setMaximumWidth(32);
            }
        }

        if (search_bar_frame_ref) {
            if (t <= 0.005) {
                search_bar_frame_ref->setVisible(true);
                search_bar_frame_ref->setMaximumWidth(16777215);
                const bool show_tb = sidebar_toggle_btn && sidebar_toggle_btn->isVisible();
                if (side_top_row_ref) side_top_row_ref->setSpacing(show_tb ? 6 : 0);
            } else {
                const int sb_w = static_cast<int>(std::round((134.0 + 106.0 * sf) * std::pow(1.0 - t, 1.35)));
                const bool show_sb = sb_w > 6;
                search_bar_frame_ref->setVisible(show_sb);
                search_bar_frame_ref->setMaximumWidth(show_sb ? sb_w : 0);
                if (side_top_row_ref) side_top_row_ref->setSpacing(show_sb ? 6 : 0);
            }
        }

        if (inline_filter_box) {
            const int fw = static_cast<int>(std::round(150.0 + 136.0 * ff));
            inline_filter_box->setFixedWidth(std::clamp(fw, 140, 320));
        }

        for (auto* lbl : side_section_labels) {
            if (!lbl) continue;
            if (t <= 0.005) {
                lbl->setVisible(true);
                lbl->setMaximumHeight(16777215);
            } else {
                const double lbl_k = std::clamp(1.0 - t * 1.45, 0.0, 1.0);
                const int lbl_h = static_cast<int>(std::round(18.0 * lbl_k));
                lbl->setVisible(lbl_h > 1);
                lbl->setMaximumHeight(lbl_h);
            }
        }

        for (auto* btn : side_nav_buttons) {
            if (btn) btn->setCollapseProgress(t);
        }

        if (daemon_btn) {
            if (t <= 0.005) {
                daemon_btn->setVisible(true);
                daemon_btn->setMaximumWidth(16777215);
                if (side_footer_ref) side_footer_ref->setSpacing(6);
            } else {
                const int db_w = static_cast<int>(std::round(98.0 * std::clamp(1.0 - t * 1.5, 0.0, 1.0)));
                const bool show_db = db_w > 12;
                daemon_btn->setVisible(show_db);
                daemon_btn->setMaximumWidth(show_db ? db_w : 0);
                if (side_footer_ref) side_footer_ref->setSpacing(show_db ? 6 : 0);
            }
        }

        if (help_rail_btn_ref) {
            if (t <= 0.005) {
                help_rail_btn_ref->setVisible(true);
                help_rail_btn_ref->setMaximumWidth(32);
            } else {
                const int hb_w = static_cast<int>(std::round(32.0 * std::clamp(1.0 - t * 1.65, 0.0, 1.0)));
                const bool show_hb = hb_w > 6;
                help_rail_btn_ref->setVisible(show_hb);
                help_rail_btn_ref->setMaximumWidth(show_hb ? hb_w : 0);
            }
        }
    };

    sidebar_anim_timer = new QTimer(sidebar);
    sidebar_anim_timer->setTimerType(Qt::PreciseTimer);
    sync_timer_to_screen_refresh(sidebar_anim_timer, sidebar);
    QObject::connect(sidebar_anim_timer, &QTimer::timeout, [apply_sidebar_anim_frame] {
        sync_timer_to_screen_refresh(sidebar_anim_timer, sidebar);
        const double target_t = sidebar_collapsed ? 1.0 : 0.0;
        const double target_sf = (search_box_focused && !sidebar_collapsed) ? 1.0 : 0.0;
        const double target_ff = filter_box_focused ? 1.0 : 0.0;
        if (reduce_motion_on) {
            sidebar_anim_t = target_t;
            sidebar_anim_vel = 0.0;
            search_focus_t = target_sf;
            search_focus_vel = 0.0;
            filter_focus_t = target_ff;
            filter_focus_vel = 0.0;
            apply_sidebar_anim_frame(sidebar_anim_t);
            sidebar_anim_timer->stop();
            return;
        }
        double dt = 1.0 / current_screen_refresh_hz(sidebar);
        if (sidebar_dt_timer.isValid()) {
            dt = std::clamp(double(sidebar_dt_timer.nsecsElapsed()) * 1e-9, 0.0005, 0.040);
        }
        sidebar_dt_timer.restart();

        // Non-linear Material 3 Expressive spatial spring-damper (rapid launch + silky non-linear ease-out tail)
        const double omega = 24.0;
        const double zeta = 0.92;
        const double err = target_t - sidebar_anim_t;
        const double accel = (omega * omega * err) - (2.0 * zeta * omega * sidebar_anim_vel);
        sidebar_anim_vel += accel * dt;
        sidebar_anim_t += sidebar_anim_vel * dt;

        const double sf_omega = 22.0;
        const double sf_zeta = 0.86;
        const double sf_err = target_sf - search_focus_t;
        const double sf_accel = (sf_omega * sf_omega * sf_err) - (2.0 * sf_zeta * sf_omega * search_focus_vel);
        search_focus_vel += sf_accel * dt;
        search_focus_t += search_focus_vel * dt;

        const double ff_err = target_ff - filter_focus_t;
        const double ff_accel = (sf_omega * sf_omega * ff_err) - (2.0 * sf_zeta * sf_omega * filter_focus_vel);
        filter_focus_vel += ff_accel * dt;
        filter_focus_t += filter_focus_vel * dt;

        const bool done_side = std::abs(target_t - sidebar_anim_t) < 0.002 && std::abs(sidebar_anim_vel) < 0.015;
        const bool done_sf = std::abs(target_sf - search_focus_t) < 0.002 && std::abs(search_focus_vel) < 0.015;
        const bool done_ff = std::abs(target_ff - filter_focus_t) < 0.002 && std::abs(filter_focus_vel) < 0.015;

        if (done_side && done_sf && done_ff) {
            sidebar_anim_t = target_t;
            sidebar_anim_vel = 0.0;
            search_focus_t = target_sf;
            search_focus_vel = 0.0;
            filter_focus_t = target_ff;
            filter_focus_vel = 0.0;
            apply_sidebar_anim_frame(sidebar_anim_t);
            sidebar_anim_timer->stop();
            return;
        }
        apply_sidebar_anim_frame(sidebar_anim_t);
    });

    // Wire Sidebar Collapse / Expand Toggle Button (Non-linear 120Hz spring animation!)
    QObject::connect(sidebar_toggle_btn, &QPushButton::clicked, [apply_sidebar_anim_frame] {
        sidebar_collapsed = !sidebar_collapsed;
        if (sidebar_toggle_btn) {
            sidebar_toggle_btn->setIcon(mineral_ui::md3_icon(
                sidebar_collapsed ? mineral_ui::M3Icon::SidebarExpand : mineral_ui::M3Icon::SidebarCollapse,
                QColor("#FFFFFF"), 16));
        }
        if (reduce_motion_on) {
            sidebar_anim_t = sidebar_collapsed ? 1.0 : 0.0;
            sidebar_anim_vel = 0.0;
            apply_sidebar_anim_frame(sidebar_anim_t);
            if (sidebar_anim_timer) sidebar_anim_timer->stop();
            return;
        }
        sidebar_dt_timer.restart();
        if (sidebar_anim_timer && !sidebar_anim_timer->isActive()) {
            sidebar_anim_timer->start();
        }
    });

    root_h_layout->addWidget(sidebar);

    // ── Right Main Content Column (Zone 2 Top Hero Banner + Zone 3 Bottom White Split Area) ──
    auto* main_col = new QWidget();
    main_col_ref = main_col;
    auto* layout = new QVBoxLayout(main_col);
    layout->setContentsMargins(0, 0, 0, 0);
    layout->setSpacing(0);
    root_h_layout->addWidget(main_col, 1);

    // Keep auxiliary pointers valid while hidden so keyboard shortcuts & snapshots stay 100% compatible
    header_bar = new QWidget(main_col);
    header_bar->setVisible(false);
    auto* search_btn = new QPushButton(header_bar);
    filter_box = new QLineEdit(header_bar);
    auto* cmd_btn = new QPushButton(header_bar);
    mode_button = new QPushButton(header_bar);
    undo_auto_btn = new QPushButton(header_bar);

    notice_bar = new QWidget();
    notice_bar->setObjectName("notice");
    auto* notice_layout = new QHBoxLayout(notice_bar);
    notice_layout->setContentsMargins(18, 6, 18, 6);
    notice_label = new ElidedLabel();
    notice_close_btn = new QPushButton("Dismiss");
    notice_close_btn->setObjectName("ghost");
    notice_layout->addWidget(notice_label, 1);
    notice_layout->addWidget(notice_close_btn);
    notice_bar->setVisible(false);

    keyboard_bar = new QWidget(main_col);
    keyboard_bar->setVisible(false);
    modular_bar = new QWidget(main_col);
    modular_bar->setVisible(false);
    queue_left_panel = new QFrame(main_col);
    queue_left_panel->setVisible(false);
    auto* ql_add_btn = new QPushButton(queue_left_panel);

    // ── ZONE 2: Top Dynamic Color-Block Hero Banner (275px normal / 100% fullscreen) ──
    hero_banner = new mineral_ui::HeroBannerWidget();
    selected_card = hero_banner;
    selected_card->setObjectName("heroCard");
    selected_card->setMinimumHeight(268);
    selected_card->setMaximumHeight(285);
    auto* hero_h_layout = new QHBoxLayout(selected_card);
    hero_h_layout->setContentsMargins(42, 20, 42, 20);
    hero_h_layout->setSpacing(36);
    hero_h_layout_ref = hero_h_layout;

    auto* hero_left = new QVBoxLayout();
    hero_left->setContentsMargins(0, 0, 0, 0);
    hero_left->setSpacing(6);

    auto* hero_top_row = new QHBoxLayout();
    hero_top_row->setContentsMargins(0, 0, 0, 0);
    hero_top_row->setSpacing(10);
    back_btn = new QPushButton("← Back to the list");
    back_btn->setObjectName("backLink");
    back_btn->setCursor(Qt::PointingHandCursor);
    back_btn->setVisible(false);
    crumbs_label = new ElidedLabel();
    crumbs_label->setObjectName("heroDesc");
    stream_badge = new QLabel();
    stream_badge->setObjectName("heroDesc");
    hero_top_row->addWidget(back_btn);
    hero_top_row->addWidget(crumbs_label, 1);
    hero_top_row->addStretch();
    hero_top_row->addWidget(stream_badge);
    hero_left->addLayout(hero_top_row);

    now_title = new ElidedLabel("Mineral");
    now_title->setObjectName("heroTitle");
    now_title->setAlignment(Qt::AlignLeft | Qt::AlignVCenter);
    now_title->setFixedHeight(44);
    hero_left->addWidget(now_title);

    now_artist = new QLabel(first_run_tip_shown
        ? QStringLiteral("Ready to play")
        : QStringLiteral("Welcome to Mineral — Click any cover or track below to start listening."));
    now_artist->setObjectName("heroDesc");
    now_artist->setWordWrap(true);
    now_artist->setMinimumWidth(0);
    now_artist->setAlignment(Qt::AlignLeft | Qt::AlignTop);
    now_artist->setMaximumWidth(580);
    hero_left->addWidget(now_artist);

    hero_left->addSpacing(4);

    // Warm blush pill tags row (Icon-only playback mode + Favourite + Lyric Track + Queue + Dynamic Color + Lyric Font Size A-/A+)
    auto* hero_tags_row = new QHBoxLayout();
    hero_tags_row->setContentsMargins(0, 0, 0, 0);
    hero_tags_row->setSpacing(10);

    mode_cycle_btn = new QPushButton();
    mode_cycle_btn->setObjectName("heroIconBtn");
    mode_cycle_btn->setCursor(Qt::PointingHandCursor);
    mode_cycle_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::ModeSequential, QColor("#FFFFFF"), 16));
    mode_cycle_btn->setIconSize(QSize(16, 16));
    mode_cycle_btn->setToolTip("Playback Mode: Sequential (Click to cycle)");

    love_btn = new QPushButton();
    love_btn->setObjectName("heroIconBtn");
    love_btn->setCursor(Qt::PointingHandCursor);
    love_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::HeartOutline, QColor("#FFFFFF"), 16));
    love_btn->setIconSize(QSize(16, 16));
    love_btn->setToolTip("Toggle Favourite (L)");

    lyric_cycle_btn = new QPushButton("Bilingual");
    lyric_cycle_btn->setObjectName("heroTag");
    lyric_cycle_btn->setCursor(Qt::PointingHandCursor);
    lyric_cycle_btn->setToolTip("Switch Lyric Track (T)");

    auto* sel_enq_btn = new QPushButton("+ Queue");
    sel_enq_btn->setObjectName("heroTag");
    sel_enq_btn->setCursor(Qt::PointingHandCursor);
    sel_enq_btn->setToolTip("Add to Play Queue");
    sel_enq_btn_ref = sel_enq_btn;

    dyn_color_btn = new QPushButton("Dynamic Color");
    dyn_color_btn->setObjectName("heroTag");
    dyn_color_btn->setCursor(Qt::PointingHandCursor);
    dyn_color_btn->setToolTip("Toggle Dynamic Cover Color Extraction / Presets");
    dyn_color_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::Palette, QColor("#FFFFFF"), 14));
    QObject::connect(dyn_color_btn, &QPushButton::clicked, [update_dyn_btn_label] {
        if (mineral_ui::dynamic_cover_color) {
            mineral_ui::dynamic_cover_color = false;
            mineral_ui::current_m3_preset = "coral";
            mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("coral");
        } else if (mineral_ui::current_m3_preset == "coral") {
            mineral_ui::current_m3_preset = "sage";
            mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("sage");
        } else if (mineral_ui::current_m3_preset == "sage") {
            mineral_ui::current_m3_preset = "lavender";
            mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("lavender");
        } else if (mineral_ui::current_m3_preset == "lavender") {
            mineral_ui::current_m3_preset = "ocean";
            mineral_ui::m3 = mineral_ui::M3ColorScheme::fromPreset("ocean");
        } else {
            mineral_ui::dynamic_cover_color = true;
            mineral_ui::current_m3_preset = "dynamic";
        }
        if (cover_widget) cover_widget->applyDynamicPalette();
        update_dyn_btn_label();
        apply_m3_theme();
    });

    lyric_font_dec_btn = new QPushButton("A−");
    lyric_font_dec_btn->setObjectName("heroTag");
    lyric_font_dec_btn->setCursor(Qt::PointingHandCursor);
    lyric_font_dec_btn->setToolTip("Decrease Lyric Font Size (-)");
    lyric_font_dec_btn->setVisible(false);
    QObject::connect(lyric_font_dec_btn, &QPushButton::clicked, [] { adjust_lyric_font_size(-2); });

    lyric_font_inc_btn = new QPushButton("A+");
    lyric_font_inc_btn->setObjectName("heroTag");
    lyric_font_inc_btn->setCursor(Qt::PointingHandCursor);
    lyric_font_inc_btn->setToolTip("Increase Lyric Font Size (+)");
    lyric_font_inc_btn->setVisible(false);
    QObject::connect(lyric_font_inc_btn, &QPushButton::clicked, [] { adjust_lyric_font_size(2); });

    hero_tags_row->addWidget(mode_cycle_btn);
    hero_tags_row->addWidget(love_btn);
    hero_tags_row->addWidget(lyric_cycle_btn);
    hero_tags_row->addWidget(sel_enq_btn);
    hero_tags_row->addWidget(dyn_color_btn);
    hero_tags_row->addWidget(lyric_font_dec_btn);
    hero_tags_row->addWidget(lyric_font_inc_btn);
    hero_tags_row->addStretch();
    hero_left->addLayout(hero_tags_row);

    // Center Synced Lyrics inside Fullscreen Hero Player (pixel-smooth scrolling + adjustable font size!)
    fullscreen_lyric_list = new QListWidget();
    fullscreen_lyric_list->setProperty("hero_mode", true);
    fullscreen_lyric_list->setMouseTracking(true);
    fullscreen_lyric_list->setSpacing(0);
    fullscreen_lyric_list->setVerticalScrollMode(QAbstractItemView::ScrollPerPixel);
    fullscreen_lyric_list->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    fullscreen_lyric_list->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    fullscreen_lyric_list->setItemDelegate(new mineral_ui::LyricDelegate(fullscreen_lyric_list));
    new InertialWheelScroller(fullscreen_lyric_list, true);
    fullscreen_lyric_list->setVisible(false);
    hero_left->addWidget(fullscreen_lyric_list, 1);

    hero_left->addStretch(0);

    // Circular Coral Play Button (52x52) + Interactive Symmetric Waveform Scrubber + Timestamps
    auto* hero_player_row = new QHBoxLayout();
    hero_player_row->setContentsMargins(0, 4, 0, 0);
    hero_player_row->setSpacing(18);

    play_button = new QPushButton();
    play_button->setObjectName("play");
    play_button->setCursor(Qt::PointingHandCursor);
    play_button->setToolTip("Play / Pause (Space)");
    hero_player_row->addWidget(play_button, 0, Qt::AlignTop);

    auto* wave_col = new QVBoxLayout();
    wave_col->setContentsMargins(0, 2, 0, 0);
    wave_col->setSpacing(4);

    viz_widget = new VizWidget();
    viz_widget->setMinimumWidth(340);
    viz_widget->setFixedHeight(40);
    wave_col->addWidget(viz_widget);

    auto* wave_meta_row = new QHBoxLayout();
    wave_meta_row->setContentsMargins(2, 0, 2, 0);
    wave_meta_row->setSpacing(8);
    pos_time_label = new QLabel("0:00");
    pos_time_label->setObjectName("heroTime");
    pos_time_label->setFixedWidth(38);
    pos_time_label->setAlignment(Qt::AlignLeft | Qt::AlignVCenter);
    auto* prev = new QPushButton();
    prev->setObjectName("heroIconBtn");
    prev->setFixedSize(28, 28);
    prev->setCursor(Qt::PointingHandCursor);
    prev->setToolTip("Previous Track (P)");
    prev_btn_ref = prev;
    auto* next = new QPushButton();
    next->setObjectName("heroIconBtn");
    next->setFixedSize(28, 28);
    next->setCursor(Qt::PointingHandCursor);
    next->setToolTip("Next Track (N)");
    next_btn_ref = next;
    next_up_label = new ElidedLabel();
    next_up_label->setObjectName("heroDesc");
    next_up_label->setMinimumWidth(0);

    // High-performance custom-painted zero-QSS VolumeBarWidget
    volume_bar = new mineral_ui::VolumeBarWidget();
    volume_bar->on_volume_change = [](int pct) {
        push_action(QString("{\"action\":\"volume\",\"pct\":%1}").arg(pct));
    };
    volume = new QSlider(Qt::Horizontal, selected_card);
    volume->setRange(0, 100);
    volume->setValue(80);
    volume->setVisible(false);

    auto* mini_btn = new QPushButton();
    mini_btn->setObjectName("heroIconBtn");
    mini_btn->setFixedSize(28, 28);
    mini_btn->setCursor(Qt::PointingHandCursor);
    mini_btn->setToolTip("Toggle Mini Player");
    mini_btn->setIcon(mineral_ui::md3_icon(mineral_ui::M3Icon::MiniPlayer, QColor("#FFFFFF"), 15));
    mini_btn_ref = mini_btn;

    auto* immersive_btn = new QPushButton();
    immersive_btn->setObjectName("heroIconBtn");
    immersive_btn->setFixedSize(28, 28);
    immersive_btn->setCursor(Qt::PointingHandCursor);
    immersive_btn->setToolTip("Toggle Fullscreen Player (Z / Esc)");
    immersive_btn_ref = immersive_btn;
    dur_time_label = new QLabel("0:00");
    dur_time_label->setObjectName("heroTime");
    dur_time_label->setFixedWidth(38);
    dur_time_label->setAlignment(Qt::AlignRight | Qt::AlignVCenter);

    wave_meta_row->addWidget(pos_time_label);
    wave_meta_row->addWidget(prev);
    wave_meta_row->addWidget(next);
    wave_meta_row->addWidget(next_up_label, 1);
    wave_meta_row->addStretch(0);
    wave_meta_row->addWidget(volume_bar);
    wave_meta_row->addWidget(mini_btn);
    wave_meta_row->addWidget(immersive_btn);
    wave_meta_row->addWidget(dur_time_label);
    wave_col->addLayout(wave_meta_row);

    hero_player_row->addLayout(wave_col, 1);
    hero_left->addLayout(hero_player_row);

    hero_h_layout->addLayout(hero_left, 1);

    cover_widget = new mineral_ui::CoverArtWidget();
    cover_widget->setBanner(hero_banner);
    cover_widget->setFixedSize(228, 228);
    hero_h_layout->addWidget(cover_widget, 0, Qt::AlignVCenter | Qt::AlignRight);

    layout->addWidget(selected_card);
    layout->addWidget(notice_bar);

    // Keep hidden auxiliary pointers initialized for full snapshot compatibility
    selected_source_badge = new QLabel(selected_card);
    selected_source_badge->setVisible(false);
    sel_title_label = new QLabel(selected_card);
    sel_title_label->setVisible(false);
    sel_sub_label = new QLabel(selected_card);
    sel_sub_label->setVisible(false);
    sel_meta_label = new QLabel(selected_card);
    sel_meta_label->setVisible(false);
    auto* sel_play_btn = new QPushButton(selected_card);
    sel_play_btn->setVisible(false);
    sel_play_btn_ref = sel_play_btn;
    sel_love_btn = new QPushButton(selected_card);
    sel_love_btn->setVisible(false);
    auto* sel_dl_btn = new QPushButton(selected_card);
    sel_dl_btn->setVisible(false);
    sel_dl_btn_ref = sel_dl_btn;
    position = new QSlider(Qt::Horizontal, selected_card);
    position->setRange(0, 1000);
    position->setVisible(false);
    auto* queue_button = new QPushButton(selected_card);
    queue_button->setVisible(false);
    queue_btn_ref = queue_button;
    mini_restore_btn = new QPushButton("Expand", selected_card);
    mini_restore_btn->setObjectName("heroTag");
    mini_restore_btn->setVisible(false);
    hero_top_row->addWidget(mini_restore_btn);

    QObject::connect(sel_play_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"activate\",\"key\":\"\"}"); });
    QObject::connect(sel_love_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"love\",\"key\":\"\"}"); });
    QObject::connect(sel_enq_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"enqueue\",\"key\":\"\"}"); });
    QObject::connect(sel_dl_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"download\",\"key\":\"\"}"); });

    // ── ZONE 3: Bottom Pure White Split Area (Left: Browse Table | Right: New Albums 2x2 Grid + Synced Lyrics) ──
    auto* body_wrap = new QWidget();
    body_wrap->setObjectName("bentoCard");
    auto* body_layout = new QVBoxLayout(body_wrap);
    body_layout->setContentsMargins(42, 26, 42, 20);
    body_layout->setSpacing(14);

    auto* bottom_split = new QHBoxLayout();
    bottom_split->setContentsMargins(0, 0, 0, 0);
    bottom_split->setSpacing(36);
    top_bento_area = body_wrap;

    // LEFT COLUMN (60%): "Browse" Clean Track Table
    center_panel = new QFrame();
    center_panel->setObjectName("bentoCard");
    auto* center_layout = new QVBoxLayout(center_panel);
    center_layout->setContentsMargins(0, 0, 0, 0);
    center_layout->setSpacing(8);

    auto* nav_header = new QHBoxLayout();
    nav_header->setContentsMargins(4, 0, 8, 4);
    nav_header->setSpacing(8);

    content_back_btn = new QPushButton("← Back");
    content_back_btn->setObjectName("chip");
    content_back_btn->setCursor(Qt::PointingHandCursor);
    content_back_btn->setVisible(false);

    heading = new ElidedLabel("Browse");
    heading->setObjectName("heading");
    heading->setFixedHeight(30);
    stats_label = new QLabel();
    stats_label->setObjectName("muted");

    inline_filter_box = new QLineEdit();
    inline_filter_box->setPlaceholderText("Filter list…");
    inline_filter_box->setClearButtonEnabled(true);
    inline_filter_box->setFixedWidth(150);
    inline_filter_box->setFixedHeight(28);

    auto* play_all_btn = new QPushButton("Play All");
    play_all_btn->setObjectName("chip");
    play_all_btn->setCursor(Qt::PointingHandCursor);
    play_all_btn_ref = play_all_btn;

    cover_tools_bar = new QWidget();
    auto* ct_layout = new QHBoxLayout(cover_tools_bar);
    ct_layout->setContentsMargins(0, 0, 0, 0);
    ct_layout->setSpacing(6);
    cover_grid_btn = new QPushButton("Grid");
    cover_grid_btn->setObjectName("chip");
    cover_grid_btn->setCursor(Qt::PointingHandCursor);
    cover_grid_btn->setCheckable(true);
    cover_list_btn = new QPushButton("List");
    cover_list_btn->setObjectName("chip");
    cover_list_btn->setCursor(Qt::PointingHandCursor);
    cover_list_btn->setCheckable(true);
    ct_layout->addWidget(cover_grid_btn);
    ct_layout->addWidget(cover_list_btn);
    cover_tools_bar->setVisible(false);

    nav_header->addWidget(content_back_btn);
    nav_header->addWidget(heading, 1);
    nav_header->addSpacing(6);
    nav_header->addWidget(stats_label);
    nav_header->addStretch(0);
    nav_header->addWidget(inline_filter_box);
    nav_header->addWidget(cover_tools_bar);
    nav_header->addWidget(play_all_btn);
    center_layout->addLayout(nav_header);

    filter_chips_bar = new QWidget();
    source_chips_layout = new QHBoxLayout(filter_chips_bar);
    source_chips_layout->setContentsMargins(4, 0, 4, 0);
    source_chips_layout->setSpacing(6);
    auto* src_lbl = new QLabel("Source:");
    src_lbl->setObjectName("muted");
    source_chips_layout->addWidget(src_lbl);
    auto* src_group = new QButtonGroup(filter_chips_bar);
    src_group->setExclusive(true);
    for (auto [sid, slabel] : std::initializer_list<std::pair<const char*, const char*>>{
             {"", "All Sources"},
             {"netease", "NetEase"},
             {"bilibili", "Bilibili"},
             {"am", "Apple Music"}}) {
        auto* btn = new QPushButton(slabel);
        btn->setObjectName("chip");
        btn->setCursor(Qt::PointingHandCursor);
        btn->setCheckable(true);
        btn->setProperty("source_id", sid);
        if (QString(sid).isEmpty()) btn->setChecked(true);
        src_group->addButton(btn);
        source_chips_layout->addWidget(btn);
        QObject::connect(btn, &QPushButton::clicked, [sid = QString(sid)] {
            push_action(QString("{\"action\":\"source_filter\",\"source\":\"%1\"}").arg(sid));
        });
    }
    source_chips_layout->addStretch();
    filter_chips_bar->setProperty("src_group_ptr", QVariant::fromValue<QObject*>(src_group));
    filter_chips_bar->setVisible(true);
    center_layout->addWidget(filter_chips_bar);

    search_tabs_bar = new QWidget();
    auto* st_layout = new QHBoxLayout(search_tabs_bar);
    st_layout->setContentsMargins(4, 0, 4, 0);
    st_layout->setSpacing(6);
    search_tab_group = new QButtonGroup(search_tabs_bar);
    search_tab_group->setExclusive(true);
    for (auto [tid, tlabel] : std::initializer_list<std::pair<const char*, const char*>>{
             {"all", "All"},
             {"song", "Tracks"},
             {"artist", "Artists"},
             {"album", "Albums"},
             {"playlist", "Playlists"}}) {
        auto* btn = new QPushButton(tlabel);
        btn->setObjectName("chip");
        btn->setCheckable(true);
        btn->setProperty("tab", tid);
        if (QString(tid) == "all") btn->setChecked(true);
        search_tab_group->addButton(btn);
        st_layout->addWidget(btn);
        QObject::connect(btn, &QPushButton::clicked, [tid = QString(tid)] {
            push_action(QString("{\"action\":\"search_tab\",\"tab\":\"%1\"}").arg(tid));
        });
    }
    st_layout->addStretch();
    summary_label = new QLabel();
    summary_label->setObjectName("muted");
    st_layout->addWidget(summary_label);
    search_tabs_bar->setVisible(false);
    center_layout->addWidget(search_tabs_bar);

    queue_tools = new QWidget();
    auto* queue_layout = new QHBoxLayout(queue_tools);
    queue_layout->setContentsMargins(4, 0, 4, 0);
    queue_layout->setSpacing(6);
    for (auto [text, action] : std::initializer_list<std::pair<const char*, const char*>>{
             {"↑", "{\"action\":\"queue_move_selected\",\"to\":\"up\"}"},
             {"↓", "{\"action\":\"queue_move_selected\",\"to\":\"down\"}"},
             {"Top", "{\"action\":\"queue_move_selected\",\"to\":\"top\"}"},
             {"Next", "{\"action\":\"queue_move_selected\",\"to\":\"next\"}"},
             {"Remove", "{\"action\":\"queue_remove_selected\"}"},
             {"Dedupe", "{\"action\":\"queue_dedupe\"}"},
             {"Clear", "{\"action\":\"queue_clear\"}"}}) {
        auto* button = new QPushButton(text);
        button->setObjectName("chip");
        button->setCursor(Qt::PointingHandCursor);
        queue_layout->addWidget(button);
        QObject::connect(button, &QPushButton::clicked, [action] { push_action(action); });
    }
    save_playlist_box = new QLineEdit();
    save_playlist_box->setPlaceholderText("Save queue as…");
    save_playlist_box->setMaximumWidth(130);
    auto* save_pl_btn = new QPushButton("Save");
    save_pl_btn->setObjectName("ghost");
    queue_layout->addWidget(save_playlist_box);
    queue_layout->addWidget(save_pl_btn);
    queue_layout->addStretch();
    queue_tools->setVisible(false);
    center_layout->addWidget(queue_tools);

    col_header_bar = new QWidget();
    col_header_bar->setObjectName("colheader");
    col_header_bar->setFixedHeight(28);
    auto* ch_layout = new QHBoxLayout(col_header_bar);
    ch_layout->setContentsMargins(66, 2, 20, 4);
    auto* c_title = new QLabel("Title");
    c_title->setObjectName("collabel");
    auto* c_artist = new QLabel("Artist");
    c_artist->setObjectName("collabel");
    auto* c_album = new QLabel("Album");
    c_album->setObjectName("collabel");
    auto* c_len = new QLabel("");
    c_len->setObjectName("collabel");
    c_len->setFixedWidth(48);
    ch_layout->addWidget(c_title, 40);
    ch_layout->addWidget(c_artist, 28);
    ch_layout->addWidget(c_album, 32);
    ch_layout->addWidget(c_len);
    center_layout->addWidget(col_header_bar);

    empty_label = new QLabel("Playback daemon is disconnected.");
    empty_label->setObjectName("muted");
    empty_label->setWordWrap(true);
    empty_label->setAlignment(Qt::AlignLeft | Qt::AlignTop);
    empty_label->setVisible(false);

    rows = new QListWidget();
    rows->setSelectionMode(QAbstractItemView::ExtendedSelection);
    rows->setContextMenuPolicy(Qt::CustomContextMenu);
    rows->setVerticalScrollMode(QAbstractItemView::ScrollPerPixel);
    rows->setSpacing(2);
    rows->setMouseTracking(true);
    rows->setItemDelegate(new mineral_ui::MineralRowDelegate(rows));
    new InertialWheelScroller(rows, false);

    touch_action_bar = new QWidget(center_panel);
    touch_action_bar->setVisible(false);
    single_drawer_bar = new QWidget(center_panel);
    single_drawer_bar->setVisible(false);

    center_layout->addWidget(empty_label, 1);
    center_layout->addWidget(rows, 1);

    bottom_split->addWidget(center_panel, 62);

    // LYRICS FULL-PANEL ("Now Playing" sidebar tab replaces song list with lyrics and hides 2x2 grid)
    lyrics_card = new QFrame();
    lyrics_card->setObjectName("bentoCard");
    auto* lyr_layout = new QVBoxLayout(lyrics_card);
    lyr_layout->setContentsMargins(0, 0, 0, 0);
    lyr_layout->setSpacing(12);
    auto* lyr_hdr = new QHBoxLayout();
    auto* lyr_tag = new QLabel("Now Playing · Synced Lyrics");
    lyr_tag->setObjectName("heading");
    auto* lyr_dec_btn = new QPushButton("A−");
    lyr_dec_btn->setObjectName("chip");
    lyr_dec_btn->setCursor(Qt::PointingHandCursor);
    lyr_dec_btn->setToolTip("Decrease Lyric Font Size (-)");
    QObject::connect(lyr_dec_btn, &QPushButton::clicked, [] { adjust_lyric_font_size(-2); });
    auto* lyr_inc_btn = new QPushButton("A+");
    lyr_inc_btn->setObjectName("chip");
    lyr_inc_btn->setCursor(Qt::PointingHandCursor);
    lyr_inc_btn->setToolTip("Increase Lyric Font Size (+)");
    QObject::connect(lyr_inc_btn, &QPushButton::clicked, [] { adjust_lyric_font_size(2); });

    lyr_hdr->addWidget(lyr_tag);
    lyr_hdr->addStretch();
    lyr_hdr->addWidget(lyr_dec_btn);
    lyr_hdr->addWidget(lyr_inc_btn);
    lyr_layout->addLayout(lyr_hdr);

    lyric_list = new QListWidget();
    lyric_list->setMouseTracking(true);
    lyric_list->setVerticalScrollMode(QAbstractItemView::ScrollPerPixel);
    lyric_list->setSpacing(0);
    lyric_list->setItemDelegate(new mineral_ui::LyricDelegate(lyric_list));
    new InertialWheelScroller(lyric_list, true);
    lyr_layout->addWidget(lyric_list, 1);
    lyrics_card->setVisible(false);
    bottom_split->addWidget(lyrics_card, 100);

    // RIGHT COLUMN (38%): "New Albums" 2x2 Large Cover Grid (4 covers, hover-only title, hidden on "Now Playing" tab)
    auto* right_col = new QWidget();
    right_col->setMinimumWidth(348);
    right_col->setMaximumWidth(364);
    auto* right_layout = new QVBoxLayout(right_col);
    right_layout->setContentsMargins(0, 0, 0, 0);
    right_layout->setSpacing(14);
    bottom_bento_area = right_col;

    auto* albums_hdr = new QLabel("New Albums");
    albums_hdr->setObjectName("heading");
    right_layout->addWidget(albums_hdr);

    lib_browser_list = new QListWidget();
    lib_browser_list->setMouseTracking(true);
    lib_browser_list->setItemDelegate(new mineral_ui::MineralRowDelegate(lib_browser_list));
    lib_browser_list->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    lib_browser_list->setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    lib_browser_list->setSpacing(0);
    lib_browser_list->setFixedSize(344, 344);
    right_layout->addWidget(lib_browser_list, 0, Qt::AlignLeft | Qt::AlignTop);
    right_layout->addStretch();

    bottom_split->addWidget(right_col, 38);
    body_layout->addLayout(bottom_split, 1);

    // Keep auxiliary pointers valid for snapshot compatibility
    spectrum_card = new QFrame(body_wrap);
    spectrum_card->setVisible(false);
    transport_card = new QFrame(body_wrap);
    transport_card->setVisible(false);

    refresh_md3_icons();

    close_box = new QWidget();
    close_box->setObjectName("notice");
    auto* close_layout = new QHBoxLayout(close_box);
    close_layout->setContentsMargins(18, 8, 18, 8);
    close_layout->addWidget(new QLabel("When closing the window (remembered after first choice):"));
    for (auto [text, choice] : std::initializer_list<std::pair<const char*, const char*>>{
        {"Keep Playing in System Tray", "ui"},
        {"Stop Playback & Quit", "stop"},
        {"Cancel", "cancel"}
    }) {
        auto* button = new QPushButton(text);
        if (QString(choice) == "stop") button->setObjectName("heroTag");
        else button->setObjectName("ghost");
        button->setCursor(Qt::PointingHandCursor);
        close_layout->addWidget(button);
        QObject::connect(button, &QPushButton::clicked, [choice] {
            const QString c(choice);
            if (c == "ui") {
                close_policy = "ui";
                push_action("{\"action\":\"close_policy\",\"policy\":\"ui\"}");
                push_action("{\"action\":\"confirm_close\",\"choice\":\"cancel\"}");
                hide_to_system_tray();
            } else if (c == "stop") {
                close_policy = "stop";
                push_action("{\"action\":\"close_policy\",\"policy\":\"stop\"}");
                push_action("{\"action\":\"confirm_close\",\"choice\":\"stop\"}");
            } else {
                push_action("{\"action\":\"confirm_close\",\"choice\":\"cancel\"}");
            }
        });
    }
    close_layout->addStretch();
    close_box->setVisible(false);
    layout->addWidget(close_box);

    layout->addWidget(body_wrap, 1);

    auto trigger_search = [] {
        now_playing_tab_active = false;
        play_queue_tab_active = false;
        if (lyrics_card) lyrics_card->setVisible(false);
        if (center_panel) center_panel->setVisible(true);
        if (bottom_bento_area) bottom_bento_area->setVisible(true);
        auto query = escape_json(search_box->text());
        push_action(QString("{\"action\":\"list_filter\",\"query\":\"\"}"));
        push_action(QString("{\"action\":\"search\",\"query\":\"%1\"}").arg(query));
    };
    QObject::connect(undo_auto_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"undo_auto\"}"); });
    QObject::connect(cmd_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"palette\"}"); });
    QObject::connect(back_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"back\"}"); });
    QObject::connect(content_back_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"back\"}"); });
    QObject::connect(play_all_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"play_container\",\"key\":\"\"}"); });
    QObject::connect(cover_grid_btn, &QPushButton::clicked, [] {
        if (current_workspace == "covers") {
            push_action("{\"action\":\"scheme\",\"scheme\":\"classic\"}");
        }
        push_action("{\"action\":\"cover_mode\",\"mode\":\"grid\"}");
    });
    QObject::connect(cover_list_btn, &QPushButton::clicked, [] {
        if (current_workspace == "covers") {
            push_action("{\"action\":\"scheme\",\"scheme\":\"classic\"}");
        }
        push_action("{\"action\":\"cover_mode\",\"mode\":\"list\"}");
    });
    QObject::connect(search_box, &QLineEdit::returnPressed, trigger_search);
    QObject::connect(search_btn, &QPushButton::clicked, trigger_search);
    QObject::connect(search_icon_btn, &QPushButton::clicked, trigger_search);
    QObject::connect(search_box, &QLineEdit::textChanged, [](const QString& text) {
        push_action(QString("{\"action\":\"list_filter\",\"query\":\"%1\"}").arg(escape_json(text)));
        if (text.isEmpty()) push_action("{\"action\":\"search\",\"query\":\"\"}");
    });
    QObject::connect(filter_box, &QLineEdit::textChanged, [](const QString& text) {
        push_action(QString("{\"action\":\"list_filter\",\"query\":\"%1\"}").arg(escape_json(text)));
    });
    QObject::connect(inline_filter_box, &QLineEdit::textChanged, [](const QString& text) {
        push_action(QString("{\"action\":\"list_filter\",\"query\":\"%1\"}").arg(escape_json(text)));
    });
    auto* search_focus_filter = new SearchFocusEventFilter(window);
    if (search_bar_frame_ref) search_bar_frame_ref->installEventFilter(search_focus_filter);
    if (search_box) search_box->installEventFilter(search_focus_filter);
    if (inline_filter_box) inline_filter_box->installEventFilter(search_focus_filter);
    QObject::connect(notice_close_btn, &QPushButton::clicked, [] {
        notice_bar->setVisible(false);
        if (last_notice_id >= 0) push_action(QString("{\"action\":\"dismiss_notice\",\"id\":%1}").arg(last_notice_id));
        else push_action("{\"action\":\"clear_notices\"}");
    });
    QObject::connect(save_pl_btn, &QPushButton::clicked, [] {
        const QString pl_name = save_playlist_box ? escape_json(save_playlist_box->text()) : QString();
        push_action(QString("{\"action\":\"save_queue_playlist\",\"name\":\"%1\"}").arg(pl_name.isEmpty() ? QStringLiteral("Queue Playlist") : pl_name));
        if (save_playlist_box) save_playlist_box->clear();
    });
    QObject::connect(ql_add_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"enqueue\",\"key\":\"\"}"); });
    QObject::connect(prev, &QPushButton::clicked, [] { push_action("{\"action\":\"prev\"}"); });
    QObject::connect(play_button, &QPushButton::clicked, [] { push_action("{\"action\":\"toggle_play\"}"); });
    QObject::connect(next, &QPushButton::clicked, [] { push_action("{\"action\":\"next\"}"); });
    QObject::connect(mode_cycle_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"cycle_mode\"}"); });
    QObject::connect(lyric_cycle_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"lyric_track\"}"); });
    QObject::connect(queue_button, &QPushButton::clicked, [] { push_action("{\"action\":\"queue_drawer\"}"); });
    QObject::connect(mini_btn, &QPushButton::clicked, [] {
        push_action(was_mini ? "{\"action\":\"back\"}" : "{\"action\":\"scheme\",\"scheme\":\"mini\"}");
    });
    QObject::connect(immersive_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"toggle_immersive\"}"); });
    QObject::connect(love_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"love\",\"key\":\"\"}"); });
    QObject::connect(mini_restore_btn, &QPushButton::clicked, [] { push_action("{\"action\":\"back\"}"); });
    QObject::connect(position, &QSlider::sliderReleased, [] { push_action(QString("{\"action\":\"seek\",\"ms\":%1}").arg(position->value())); });
    QObject::connect(volume, &QSlider::sliderReleased, [] { push_action(QString("{\"action\":\"volume\",\"pct\":%1}").arg(volume->value())); });

    auto open_daemon_lua_editor = [](const QString& initial_lua) {
        auto* dlg = new QDialog(window);
        dlg->setWindowTitle("Edit Mineral Daemon Script (~/.config/mineral/daemon.lua)");
        dlg->setModal(true);
        dlg->setStyleSheet(shell_style(touch_on));
        dlg->resize(720, 620);

        auto* v = new QVBoxLayout(dlg);
        v->setContentsMargins(22, 18, 22, 18);
        v->setSpacing(10);

        auto* title_lbl = new QLabel("Mineral Daemon Configuration (daemon.lua)");
        title_lbl->setObjectName("heading");
        v->addWidget(title_lbl);

        auto* sub_lbl = new QLabel("Changes saved here are written directly to ~/.config/mineral/daemon.lua and hot-reloaded by mineral serve within 1 second.");
        sub_lbl->setObjectName("muted");
        sub_lbl->setWordWrap(true);
        v->addWidget(sub_lbl);

        auto* editor = new QPlainTextEdit();
        editor->setPlainText(initial_lua);
        QFont mono("JetBrains Mono, Monospace, Courier New");
        mono.setStyleHint(QFont::Monospace);
        mono.setPixelSize(13);
        editor->setFont(mono);
        editor->setStyleSheet(
            "QPlainTextEdit { background: #FAF7F5; color: #2B2625; border: 1px solid #E4DCD6; border-radius: 10px; padding: 10px; }"
        );
        v->addWidget(editor, 1);

        auto* btn_row = new QHBoxLayout();
        auto* reset_btn = new QPushButton("Reset All to Defaults");
        reset_btn->setObjectName("ghost");
        reset_btn->setCursor(Qt::PointingHandCursor);
        auto* restart_btn = new QPushButton("Restart Daemon");
        restart_btn->setObjectName("ghost");
        restart_btn->setCursor(Qt::PointingHandCursor);
        auto* cancel_btn = new QPushButton("Cancel");
        cancel_btn->setObjectName("ghost");
        cancel_btn->setCursor(Qt::PointingHandCursor);
        auto* save_btn = new QPushButton("Save & Hot-Reload");
        save_btn->setObjectName("chip");
        save_btn->setCursor(Qt::PointingHandCursor);

        btn_row->addWidget(reset_btn);
        btn_row->addWidget(restart_btn);
        btn_row->addStretch(1);
        btn_row->addWidget(cancel_btn);
        btn_row->addWidget(save_btn);
        v->addLayout(btn_row);

        QObject::connect(reset_btn, &QPushButton::clicked, [dlg] {
            push_action("{\"action\":\"daemon_config_reset\"}");
            dlg->accept();
        });
        QObject::connect(restart_btn, &QPushButton::clicked, [] {
            push_action("{\"action\":\"restart_daemon\"}");
        });
        QObject::connect(cancel_btn, &QPushButton::clicked, dlg, &QDialog::reject);
        QObject::connect(save_btn, &QPushButton::clicked, [dlg, editor] {
            const QString lua = escape_json(editor->toPlainText());
            push_action(QString("{\"action\":\"daemon_save_raw_lua\",\"lua\":\"%1\"}").arg(lua));
            dlg->accept();
        });
        dlg->exec();
        dlg->deleteLater();
    };

    auto open_daemon_setting_dialog = [](const QString& field_key,
                                         const QString& title,
                                         const QString& hint,
                                         const QString& ctrl_type,
                                         const QString& raw_val) {
        auto* dlg = new QDialog(window);
        dlg->setWindowTitle("Configure Daemon Setting — " + title);
        dlg->setModal(true);
        dlg->setStyleSheet(shell_style(touch_on));
        dlg->resize(540, 250);

        auto* v = new QVBoxLayout(dlg);
        v->setContentsMargins(22, 18, 22, 18);
        v->setSpacing(10);

        auto* title_lbl = new QLabel(title);
        title_lbl->setObjectName("heading");
        v->addWidget(title_lbl);

        auto* hint_lbl = new QLabel(hint);
        hint_lbl->setObjectName("muted");
        hint_lbl->setWordWrap(true);
        v->addWidget(hint_lbl);

        auto* input_row = new QHBoxLayout();
        input_row->setSpacing(8);
        auto* edit = new QLineEdit(raw_val);
        edit->setClearButtonEnabled(true);
        edit->setMinimumHeight(34);
        if (ctrl_type == "dir") {
            edit->setPlaceholderText("Leave empty or 'nil' for default (~/Music/mineral)");
        } else if (ctrl_type == "dirs") {
            edit->setPlaceholderText("Comma-separated folder paths, e.g. /home/user/Music, /mnt/flac");
        } else if (ctrl_type == "proxy") {
            edit->setPlaceholderText("false (Direct) or http://127.0.0.1:7890 / socks5://127.0.0.1:1080");
        } else if (ctrl_type == "bytes") {
            edit->setPlaceholderText("e.g. 10GB, 512MB, 256KB, or exact byte count");
        }
        input_row->addWidget(edit, 1);

        if (ctrl_type == "dir") {
            auto* browse_btn = new QPushButton("Browse Folder…");
            browse_btn->setObjectName("chip");
            browse_btn->setCursor(Qt::PointingHandCursor);
            input_row->addWidget(browse_btn);
            QObject::connect(browse_btn, &QPushButton::clicked, [dlg, edit] {
                const QString dir = QFileDialog::getExistingDirectory(dlg, "Select Download Target Directory", edit->text());
                if (!dir.isEmpty()) edit->setText(dir);
            });
        } else if (ctrl_type == "dirs") {
            auto* add_dir_btn = new QPushButton("+ Add Folder…");
            add_dir_btn->setObjectName("chip");
            add_dir_btn->setCursor(Qt::PointingHandCursor);
            input_row->addWidget(add_dir_btn);
            QObject::connect(add_dir_btn, &QPushButton::clicked, [dlg, edit] {
                const QString dir = QFileDialog::getExistingDirectory(dlg, "Add Local Music Folder");
                if (!dir.isEmpty()) {
                    const QString cur = edit->text().trimmed();
                    edit->setText(cur.isEmpty() ? dir : (cur + ", " + dir));
                }
            });
        }
        v->addLayout(input_row);

        // Quick preset chips for common fields
        QStringList presets;
        if (field_key == "audio.volume") {
            presets = {"25", "50", "75", "85", "100"};
        } else if (field_key == "cache.audio_capacity") {
            presets = {"1GB", "5GB", "10GB", "20GB", "50GB"};
        } else if (field_key == "audio.prefetch_bytes") {
            presets = {"128KB", "256KB", "512KB", "1MB"};
        } else if (ctrl_type == "proxy") {
            presets = {"false", "http://127.0.0.1:7890", "socks5://127.0.0.1:1080"};
        } else if (field_key == "stats.retention_days") {
            presets = {"false", "30", "90", "180", "365"};
        } else if (ctrl_type == "dir") {
            presets = {"nil"};
        }
        if (!presets.isEmpty()) {
            auto* preset_row = new QHBoxLayout();
            preset_row->setSpacing(6);
            auto* p_lbl = new QLabel("Presets:");
            p_lbl->setObjectName("muted");
            preset_row->addWidget(p_lbl);
            for (const QString& pval : presets) {
                auto* pb = new QPushButton(pval);
                pb->setObjectName("ghost");
                pb->setCursor(Qt::PointingHandCursor);
                preset_row->addWidget(pb);
                QObject::connect(pb, &QPushButton::clicked, [edit, pval] {
                    edit->setText(pval);
                });
            }
            preset_row->addStretch(1);
            v->addLayout(preset_row);
        }

        v->addStretch(1);

        auto* btn_row = new QHBoxLayout();
        auto* def_btn = new QPushButton("Reset Default");
        def_btn->setObjectName("ghost");
        def_btn->setCursor(Qt::PointingHandCursor);
        auto* cancel_btn = new QPushButton("Cancel");
        cancel_btn->setObjectName("ghost");
        cancel_btn->setCursor(Qt::PointingHandCursor);
        auto* apply_btn = new QPushButton("Save & Apply");
        apply_btn->setObjectName("chip");
        apply_btn->setCursor(Qt::PointingHandCursor);
        apply_btn->setDefault(true);

        btn_row->addWidget(def_btn);
        btn_row->addStretch(1);
        btn_row->addWidget(cancel_btn);
        btn_row->addWidget(apply_btn);
        v->addLayout(btn_row);

        QObject::connect(def_btn, &QPushButton::clicked, [dlg, field_key] {
            push_action(QString("{\"action\":\"daemon_config_set\",\"key\":\"%1\",\"value\":\"__reset__\"}")
                            .arg(escape_json(field_key)));
            dlg->accept();
        });
        QObject::connect(cancel_btn, &QPushButton::clicked, dlg, &QDialog::reject);
        auto submit = [dlg, edit, field_key] {
            push_action(QString("{\"action\":\"daemon_config_set\",\"key\":\"%1\",\"value\":\"%2\"}")
                            .arg(escape_json(field_key), escape_json(edit->text().trimmed())));
            dlg->accept();
        };
        QObject::connect(apply_btn, &QPushButton::clicked, submit);
        QObject::connect(edit, &QLineEdit::returnPressed, submit);

        edit->setFocus();
        edit->selectAll();
        dlg->exec();
        dlg->deleteLater();
    };

    auto handle_settings_key = [open_daemon_lua_editor, open_daemon_setting_dialog](QListWidgetItem* item) -> bool {
        if (!item) return false;
        const QString key = item->data(mineral_ui::RoleKey).toString();
        if (key == "settings:close") {
            const auto next_policy = close_policy == "ask" ? "ui" : close_policy == "ui" ? "stop" : "ask";
            push_action(QString("{\"action\":\"close_policy\",\"policy\":\"%1\"}").arg(next_policy));
            return true;
        }
        if (key == "settings:adapt") { push_action(QString("{\"action\":\"auto_adapt\",\"on\":%1}").arg(auto_adapt_on ? "false" : "true")); return true; }
        if (key == "settings:lock") { push_action(QString("{\"action\":\"lock\",\"on\":%1}").arg(locked_on ? "false" : "true")); return true; }
        if (key == "settings:motion") { push_action(QString("{\"action\":\"reduce_motion\",\"on\":%1}").arg(reduce_motion_on ? "false" : "true")); return true; }
        if (key == "settings:reset-layout") { push_action("{\"action\":\"reset_layout\"}"); return true; }
        if (key == "settings:reset-modes") { push_action("{\"action\":\"reset_modes\"}"); return true; }
        if (key == "settings:daemon") { push_action(daemon_connected ? "{\"action\":\"connect\"}" : "{\"action\":\"launch_daemon\"}"); return true; }
        if (key == "settings:daemon:restart") { push_action("{\"action\":\"restart_daemon\"}"); return true; }
        if (key == "settings:daemon:audio_output") { push_action("{\"action\":\"cycle_audio_output\"}"); return true; }
        if (key == "settings:daemon:reset_defaults") { push_action("{\"action\":\"daemon_config_reset\"}"); return true; }
        if (key == "settings:login:am") { open_am_token_dialog(); return true; }
        if (key == "settings:login:netease") { open_qr_login_dialog(QStringLiteral("netease")); return true; }
        if (key == "settings:login:bilibili") { open_qr_login_dialog(QStringLiteral("bilibili")); return true; }
        if (key == "source:am"
            && item->data(mineral_ui::RoleSubtitle).toString().contains(QStringLiteral("Not logged in"), Qt::CaseInsensitive)) {
            open_am_token_dialog();
            return true;
        }
        if ((key == "source:netease" || key == "source:bilibili")
            && item->data(mineral_ui::RoleSubtitle).toString().contains(QStringLiteral("Not logged in"), Qt::CaseInsensitive)) {
            open_qr_login_dialog(key.mid(7));
            return true;
        }
        if (key == "settings:daemon:edit_lua") {
            const QString src = item->data(mineral_ui::RoleSource).toString();
            const QString lua = src.startsWith("lua|") ? src.mid(4) : src;
            open_daemon_lua_editor(lua);
            return true;
        }
        if (key.startsWith("settings:daemon:")) {
            const QString field_key = key.mid(16);
            const QString src = item->data(mineral_ui::RoleSource).toString();
            const int sep = src.indexOf('|');
            const QString ctrl_type = (sep >= 0) ? src.left(sep) : QStringLiteral("text");
            const QString raw_val = (sep >= 0) ? src.mid(sep + 1) : src;
            if (ctrl_type == "cycle" || ctrl_type == "toggle") {
                push_action(QString("{\"action\":\"daemon_config_set\",\"key\":\"%1\",\"value\":\"__cycle__\"}")
                                .arg(escape_json(field_key)));
                return true;
            }
            const QString title = item->data(mineral_ui::RoleTitle).toString();
            const QString hint = item->data(mineral_ui::RoleAlbum).toString();
            open_daemon_setting_dialog(field_key, title, hint, ctrl_type, raw_val);
            return true;
        }
        return false;
    };

    QObject::connect(rows, &QListWidget::itemSelectionChanged, [] {
        const auto selected = rows->selectedItems();
        if (selected.isEmpty()) return;
        QString json_keys = "[";
        for (int i = 0; i < selected.size(); ++i) {
            if (i > 0) json_keys += ",";
            json_keys += "\"" + escape_json(selected[i]->data(mineral_ui::RoleKey).toString()) + "\"";
        }
        json_keys += "]";
        push_action(QString("{\"action\":\"select_many\",\"keys\":%1}").arg(json_keys));
    });

    QObject::connect(rows, &QListWidget::itemClicked, [handle_settings_key](QListWidgetItem* item) {
        if (!item) return;
        const auto key = item->data(mineral_ui::RoleKey).toString();
        const auto kind = item->data(mineral_ui::RoleKind).toString();
        const QString esc_key = escape_json(key);
        push_action(QString("{\"action\":\"select\",\"key\":\"%1\"}").arg(esc_key));
        if (handle_settings_key(item)) return;

        // Direct single-click mouse actions on track row (cover thumbnail or right-side Play / Favourite / Queue buttons)
        if (!item->data(mineral_ui::RoleGridMode).toBool() && rows && rows->viewport()) {
            const QPoint vp_pos = rows->viewport()->mapFromGlobal(QCursor::pos());
            const QRect item_rect = rows->visualItemRect(item);
            const int rx = vp_pos.x() - item_rect.left();
            const int right_dist = item_rect.right() - vp_pos.x();
            if (kind != "source" && rx >= 10 && rx <= 62) {
                push_action(QString("{\"action\":\"activate\",\"key\":\"%1\"}").arg(esc_key));
                return;
            }
            if ((kind == "track" || kind == "song" || kind.isEmpty()) && right_dist >= 12 && right_dist <= 112) {
                if (right_dist >= 76) {
                    push_action(QString("{\"action\":\"activate\",\"key\":\"%1\"}").arg(esc_key));
                } else if (right_dist >= 44) {
                    push_action(QString("{\"action\":\"love\",\"key\":\"%1\"}").arg(esc_key));
                } else {
                    push_action(QString("{\"action\":\"enqueue\",\"key\":\"%1\"}").arg(esc_key));
                }
                return;
            }
        }

        if (single_mode_on || touch_on || kind == "source" || kind == "nav" || kind == "playlist" || kind == "album" || kind == "artist" || current_content_kind == "grid" || item->data(mineral_ui::RoleGridMode).toBool()) {
            push_action(QString("{\"action\":\"activate\",\"key\":\"%1\"}").arg(esc_key));
        }
    });

    QObject::connect(rows, &QListWidget::itemDoubleClicked, [handle_settings_key](QListWidgetItem* item) {
        if (!item) return;
        const auto key = item->data(mineral_ui::RoleKey).toString();
        if (handle_settings_key(item)) return;
        push_action(QString("{\"action\":\"activate\",\"key\":\"%1\"}").arg(escape_json(key)));
    });

    QObject::connect(rows, &QWidget::customContextMenuRequested, [handle_settings_key](const QPoint& pos) {
        auto* item = rows->itemAt(pos);
        if (!item) return;
        const auto raw_key = item->data(mineral_ui::RoleKey).toString();
        const auto key = escape_json(raw_key);
        const auto kind = item->data(mineral_ui::RoleKind).toString();
        push_action(QString("{\"action\":\"select\",\"key\":\"%1\"}").arg(key));
        QMenu ctx(rows);
        if (kind == "setting" || raw_key.startsWith("settings:")) {
            ctx.addAction("Configure / Toggle Setting", [item, handle_settings_key] {
                handle_settings_key(item);
            });
            if (raw_key.startsWith("settings:daemon:")
                && raw_key != "settings:daemon:restart"
                && raw_key != "settings:daemon:audio_output"
                && raw_key != "settings:daemon:edit_lua"
                && raw_key != "settings:daemon:reset_defaults") {
                const QString field = escape_json(raw_key.mid(16));
                ctx.addAction("Reset Setting to Default", [field] {
                    push_action(QString("{\"action\":\"daemon_config_set\",\"key\":\"%1\",\"value\":\"__reset__\"}").arg(field));
                });
            }
            ctx.addSeparator();
            ctx.addAction("Restart Mineral Daemon", [] { push_action("{\"action\":\"restart_daemon\"}"); });
            ctx.addAction("Switch Audio Output Device", [] { push_action("{\"action\":\"cycle_audio_output\"}"); });
            ctx.addAction("Reset All Daemon Settings to Default", [] { push_action("{\"action\":\"daemon_config_reset\"}"); });
            ctx.exec(rows->viewport()->mapToGlobal(pos));
            return;
        }
        if (kind == "playlist" || kind == "album" || kind == "artist" || kind == "source") {
            if (raw_key == "source:am") {
                ctx.addAction("Token Login / Switch Account…", [] { open_am_token_dialog(); });
                ctx.addSeparator();
            } else if (raw_key == "source:netease" || raw_key == "source:bilibili") {
                const QString ch = raw_key.mid(7);
                ctx.addAction("QR Code Login / Switch Account…", [ch] { open_qr_login_dialog(ch); });
                ctx.addSeparator();
            }
            ctx.addAction("Open Details", [key] { push_action(QString("{\"action\":\"activate\",\"key\":\"%1\"}").arg(key)); });
            ctx.addAction("Play All Tracks", [key] { push_action(QString("{\"action\":\"play_container\",\"key\":\"%1\"}").arg(key)); });
        } else {
            ctx.addAction("Play Now", [key] { push_action(QString("{\"action\":\"activate\",\"key\":\"%1\"}").arg(key)); });
            ctx.addAction("Play Next", [key] { push_action(QString("{\"action\":\"play_next\",\"key\":\"%1\"}").arg(key)); });
            ctx.addAction("Add to Queue", [key] { push_action(QString("{\"action\":\"enqueue\",\"key\":\"%1\"}").arg(key)); });
            ctx.addAction("Toggle Favourite", [key] { push_action(QString("{\"action\":\"love\",\"key\":\"%1\"}").arg(key)); });
            ctx.addAction("Download Locally", [key] { push_action(QString("{\"action\":\"download\",\"key\":\"%1\"}").arg(key)); });
            ctx.addAction("Copy Info", [key] { push_action(QString("{\"action\":\"copy\",\"key\":\"%1\"}").arg(key)); });
        }
        if (play_queue_tab_active || current_content_kind == "queue") {
            ctx.addSeparator();
            ctx.addAction("Move Up in Queue", [] { push_action("{\"action\":\"queue_move_selected\",\"to\":\"up\"}"); });
            ctx.addAction("Move Down in Queue", [] { push_action("{\"action\":\"queue_move_selected\",\"to\":\"down\"}"); });
            ctx.addAction("Move to Next Up", [] { push_action("{\"action\":\"queue_move_selected\",\"to\":\"next\"}"); });
            ctx.addAction("Remove from Queue", [] { push_action("{\"action\":\"queue_remove_selected\"}"); });
        }
        ctx.exec(rows->viewport()->mapToGlobal(pos));
    });

    QObject::connect(lib_browser_list, &QListWidget::itemClicked, [](QListWidgetItem* item) {
        if (!item) return;
        now_playing_tab_active = false;
        play_queue_tab_active = false;
        if (lyrics_card) lyrics_card->setVisible(false);
        if (center_panel) center_panel->setVisible(true);
        if (bottom_bento_area) bottom_bento_area->setVisible(true);
        const auto key = escape_json(item->data(mineral_ui::RoleKey).toString());
        const auto kind = item->data(mineral_ui::RoleKind).toString();
        push_action(QString("{\"action\":\"select\",\"key\":\"%1\"}").arg(key));
        if (kind == "album" || kind == "playlist" || kind == "artist") {
            push_action(QString("{\"action\":\"open\",\"key\":\"%1\",\"kind\":\"%2\"}").arg(key, escape_json(kind)));
        } else {
            push_action(QString("{\"action\":\"activate\",\"key\":\"%1\"}").arg(key));
        }
    });
    QObject::connect(lib_browser_list, &QListWidget::itemDoubleClicked, [](QListWidgetItem* item) {
        if (!item) return;
        now_playing_tab_active = false;
        play_queue_tab_active = false;
        if (lyrics_card) lyrics_card->setVisible(false);
        if (center_panel) center_panel->setVisible(true);
        if (bottom_bento_area) bottom_bento_area->setVisible(true);
        const auto key = escape_json(item->data(mineral_ui::RoleKey).toString());
        push_action(QString("{\"action\":\"play_container\",\"key\":\"%1\"}").arg(key));
    });

    QObject::connect(lyric_list, &QListWidget::itemClicked, [](QListWidgetItem* item) {
        push_action(QString("{\"action\":\"lyric_seek\",\"ms\":%1}").arg(item->data(Qt::UserRole).toString()));
    });
    QObject::connect(fullscreen_lyric_list, &QListWidget::itemClicked, [](QListWidgetItem* item) {
        push_action(QString("{\"action\":\"lyric_seek\",\"ms\":%1}").arg(item->data(Qt::UserRole).toString()));
    });

    mode_dialog = make_card("Mode Center", "Filter…", &mode_search_box, &mode_list, true);
    palette_dialog = make_card("Command Palette & Quick Jump", "Type > for commands, or search tracks…", &palette_search_box, &palette_list, false);
    if (palette_list) new InertialWheelScroller(palette_list, false);
    QObject::connect(palette_search_box, &QLineEdit::textChanged, [](const QString& text) {
        push_action(QString("{\"action\":\"palette_query\",\"query\":\"%1\"}").arg(escape_json(text)));
    });
    QObject::connect(palette_search_box, &QLineEdit::returnPressed, [] {
        if (palette_list && palette_list->count() > 0) {
            auto* first = palette_list->item(0);
            push_action(QString("{\"action\":\"run_command\",\"command\":\"%1\"}").arg(escape_json(first->data(Qt::UserRole).toString())));
        }
    });
    QObject::connect(palette_dialog, &QDialog::rejected, [] { push_action("{\"action\":\"palette\",\"open\":false}"); });
    QObject::connect(palette_list, &QListWidget::itemClicked, [](QListWidgetItem* item) {
        push_action(QString("{\"action\":\"run_command\",\"command\":\"%1\"}").arg(escape_json(item->data(Qt::UserRole).toString())));
    });

    help_dialog = new QDialog(window);
    help_dialog->setWindowTitle("Keyboard Shortcuts & Interaction Guide");
    help_dialog->resize(520, 440);
    help_dialog->setStyleSheet(shell_style());
    auto* help_layout = new QVBoxLayout(help_dialog);
    auto* help_title = new QLabel("Global Shortcuts & Mouse Controls");
    help_title->setObjectName("heading");
    auto* help_text = new QLabel(
        "• Space: Play / Pause\n"
        "• Click Cover or Hover ▶ / ♡ / +: Play track, Favourite, or Add to Queue\n"
        "• Double-Click Row / Enter: Play track or open album/playlist details\n"
        "• + / -: Increase / Decrease Fullscreen & Synced Lyric font size\n"
        "• j / k: Move selection Down / Up\n"
        "• N / P: Next / Previous track\n"
        "• L / D: Favourite / Download selected track\n"
        "• /: Focus inline list filter\n"
        "• T: Cycle lyric track (Bilingual / Translation / Romanized)\n"
        "• M: Cycle playback mode icon (Sequential / Repeat All / Repeat One / Shuffle)\n"
        "• Z / Esc: Enter Fullscreen Player / Go back or exit Fullscreen\n"
        "• Ctrl + K: Open Command Palette"
    );
    help_text->setWordWrap(true);
    help_layout->addWidget(help_title);
    help_layout->addWidget(help_text, 1);
    QObject::connect(help_dialog, &QDialog::rejected, [] { push_action("{\"action\":\"help\",\"open\":false}"); });

    tray_icon = new QSystemTrayIcon(make_tray_icon(), window);
    tray_icon->setToolTip("Mineral Music Player");
    auto* tray_menu = new QMenu(window);
    tray_menu->addAction("Show Mineral", [] { restore_from_system_tray(); });
    tray_menu->addAction("Toggle Mini Player", [] {
        restore_from_system_tray();
        push_action(was_mini ? "{\"action\":\"back\"}" : "{\"action\":\"scheme\",\"scheme\":\"mini\"}");
    });
    tray_menu->addSeparator();
    tray_menu->addAction("Play / Pause", [] { push_action("{\"action\":\"toggle_play\"}"); });
    tray_menu->addAction("Previous Track", [] { push_action("{\"action\":\"prev\"}"); });
    tray_menu->addAction("Next Track", [] { push_action("{\"action\":\"next\"}"); });
    tray_menu->addSeparator();
    tray_menu->addAction("Hide to System Tray", [] { hide_to_system_tray(); });
    tray_menu->addAction("Quit Mineral (Stop Playback)", [] {
        allow_quit = true;
        push_action("{\"action\":\"confirm_close\",\"choice\":\"stop\"}");
    });
    tray_icon->setContextMenu(tray_menu);
    QObject::connect(tray_icon, &QSystemTrayIcon::activated, [](QSystemTrayIcon::ActivationReason reason) {
        if (reason == QSystemTrayIcon::Trigger || reason == QSystemTrayIcon::DoubleClick || reason == QSystemTrayIcon::MiddleClick) {
            restore_from_system_tray();
        }
    });
    if (QSystemTrayIcon::isSystemTrayAvailable()) {
        tray_icon->show();
    }

    window->setCentralWidget(root);
    window->show();
    normal_win_size = window->size();
    normal_win_pos = window->pos();
    normal_pos_valid = true;
    normal_win_geom = window->saveGeometry();
}

extern "C" int qt_process() {
    if (app == nullptr) return 0;
    app->processEvents();
    return window != nullptr && (window->isVisible() || !allow_quit) ? 1 : 0;
}

extern "C" void qt_tick_playback(int playing_int, long long pos_ms, long long dur_ms, const float* peaks, int peaks_len) {
    if (window == nullptr) return;
    const bool playing = (playing_int != 0);
    viz_pos_ms = static_cast<qint64>(pos_ms);
    if (dur_ms > 0) viz_dur_ms = static_cast<qint64>(dur_ms);

    if (playing != last_playing_state) {
        last_playing_state = playing;
        if (play_button) {
            play_button->setIcon(mineral_ui::md3_icon(
                playing ? mineral_ui::M3Icon::Pause : mineral_ui::M3Icon::Play, QColor("#FFFFFF"), 20));
        }
    }
    if (viz_widget) viz_widget->notifyPlaybackState(playing);
    if (position) {
        position->setRange(0, std::max(1, static_cast<int>(viz_dur_ms)));
        if (!position->isSliderDown()) position->setValue(static_cast<int>(viz_pos_ms));
    }
    if (pos_time_label) pos_time_label->setText(format_ms(viz_pos_ms));
    if (dur_time_label && viz_dur_ms > 0) dur_time_label->setText(format_ms(viz_dur_ms));

    viz_peaks.clear();
    float peak_energy = 0.0f;
    if (peaks != nullptr && peaks_len > 0) {
        viz_peaks.assign(peaks, peaks + peaks_len);
        for (int i = 0; i < peaks_len; ++i) {
            peak_energy += std::abs(peaks[i]);
        }
        peak_energy /= static_cast<float>(peaks_len);
    }
    if (hero_banner) hero_banner->tickDynamic(playing, peak_energy);
    sync_active_lyric_from_pos(viz_pos_ms);
}

extern "C" void qt_set_snapshot(const char* json) {
    const auto doc = QJsonDocument::fromJson(QByteArray(json));
    if (!doc.isObject() || window == nullptr) return;
    const auto snap = doc.object();
    const int generation = snap.value("generation").toInt();
    allow_quit = snap.value("quit").toBool();
    if (allow_quit) {
        window->close();
        app->quit();
        return;
    }
    const bool full = generation != last_generation;
    last_generation = generation;
    const bool playing = snap.value("playing").toBool();
    const qint64 pos_ms = snap.value("position_ms").toVariant().toLongLong();
    const qint64 dur_ms = snap.value("duration_ms").toVariant().toLongLong();
    viz_pos_ms = pos_ms;
    viz_dur_ms = dur_ms;

    if (playing != last_playing_state) {
        last_playing_state = playing;
        play_button->setIcon(mineral_ui::md3_icon(
            playing ? mineral_ui::M3Icon::Pause : mineral_ui::M3Icon::Play, QColor("#FFFFFF"), 20));
    }
    if (viz_widget) viz_widget->notifyPlaybackState(playing);
    play_button->setText(QString());
    position->setRange(0, std::max(1, static_cast<int>(dur_ms)));
    if (!position->isSliderDown()) position->setValue(static_cast<int>(pos_ms));
    if (pos_time_label) pos_time_label->setText(format_ms(pos_ms));
    if (dur_time_label) dur_time_label->setText(format_ms(dur_ms));

    int active_lyric_row = -1;
    QString active_lyric_line;
    QString lyrics_sig;
    const auto lyrics_arr = snap.value("lyrics").toArray();
    for (int i = 0; i < lyrics_arr.size(); ++i) {
        const auto item_obj = lyrics_arr[i].toObject();
        const auto l_text = item_obj.value("text").toString();
        const auto l_sub = item_obj.value("sub_text").toString();
        if (item_obj.value("active").toBool()) {
            active_lyric_row = i;
            active_lyric_line = l_sub.isEmpty() ? l_text : (l_text + "  ·  " + l_sub);
        }
        lyrics_sig += l_text + QChar('\t') + l_sub + QChar('\n');
    }
    if (lyric_list->property("lyrics_sig").toString() != lyrics_sig) {
        lyric_list->setProperty("lyrics_sig", lyrics_sig);
        lyric_list->setProperty("active_row", -2);
        lyric_list->setProperty("smooth_row", 0.0);
        lyric_list->setProperty("smooth_scroll_y", 0.0);
        lyric_list->setProperty("subpixel_y", 0.0);
        lyric_list->clear();
        if (fullscreen_lyric_list) {
            fullscreen_lyric_list->setProperty("active_row", -2);
            fullscreen_lyric_list->setProperty("smooth_row", 0.0);
            fullscreen_lyric_list->setProperty("smooth_scroll_y", 0.0);
            fullscreen_lyric_list->setProperty("subpixel_y", 0.0);
            fullscreen_lyric_list->clear();
        }
        const int norm_px = mineral_ui::normal_lyric_font_px;
        const int norm_sub = std::max(11, norm_px - 3);
        const int fs_px = mineral_ui::fullscreen_lyric_font_px;
        const int fs_sub = std::max(11, fs_px - 5);
        for (int i = 0; i < lyrics_arr.size(); ++i) {
            const auto item_obj = lyrics_arr[i].toObject();
            const auto text = item_obj.value("text").toString();
            const auto sub_text = item_obj.value("sub_text").toString();
            const qint64 t_val = (item_obj.value("t").isNull() || item_obj.value("t").isUndefined())
                ? -1LL
                : item_obj.value("t").toVariant().toLongLong();
            const QString t_str = QString::number(t_val);

            auto* item = new QListWidgetItem(text);
            item->setData(Qt::UserRole, t_str);
            item->setData(mineral_ui::RoleTitle, text);
            item->setData(mineral_ui::RoleSubtitle, sub_text);
            item->setData(mineral_ui::RoleCurrent, false);
            item->setSizeHint(QSize(0, sub_text.isEmpty() ? (norm_px + 22) : (norm_px + norm_sub + 26)));
            lyric_list->addItem(item);

            if (fullscreen_lyric_list) {
                auto* fs_item = new QListWidgetItem(text);
                fs_item->setData(Qt::UserRole, t_str);
                fs_item->setData(mineral_ui::RoleTitle, text);
                fs_item->setData(mineral_ui::RoleSubtitle, sub_text);
                fs_item->setData(mineral_ui::RoleCurrent, false);
                fs_item->setSizeHint(QSize(0, sub_text.isEmpty() ? (fs_px + 26) : (fs_px + fs_sub + 32)));
                fullscreen_lyric_list->addItem(fs_item);
            }
        }
    }
    const int stored_row = lyric_list->property("active_row").isValid() ? lyric_list->property("active_row").toInt() : -2;
    if (stored_row != active_lyric_row) {
        const int prev_row = stored_row;
        lyric_list->setProperty("active_row", active_lyric_row);
        if (prev_row < 0 && active_lyric_row >= 0) {
            lyric_list->setProperty("smooth_row", double(active_lyric_row));
        }
        if (prev_row >= 0 && prev_row < lyric_list->count()) {
            lyric_list->item(prev_row)->setData(mineral_ui::RoleCurrent, false);
        }
        if (active_lyric_row >= 0 && active_lyric_row < lyric_list->count()) {
            lyric_list->item(active_lyric_row)->setData(mineral_ui::RoleCurrent, true);
        }

        if (fullscreen_lyric_list) {
            fullscreen_lyric_list->setProperty("active_row", active_lyric_row);
            if (prev_row < 0 && active_lyric_row >= 0) {
                fullscreen_lyric_list->setProperty("smooth_row", double(active_lyric_row));
            }
            if (prev_row >= 0 && prev_row < fullscreen_lyric_list->count()) {
                fullscreen_lyric_list->item(prev_row)->setData(mineral_ui::RoleCurrent, false);
            }
            if (active_lyric_row >= 0 && active_lyric_row < fullscreen_lyric_list->count()) {
                fullscreen_lyric_list->item(active_lyric_row)->setData(mineral_ui::RoleCurrent, true);
            }
        }
        smooth_scroll_lyrics(active_lyric_row);
    }

    viz_style = snap.value("viz_style").toString();
    viz_peaks.clear();
    float peak_energy = 0.0f;
    for (const auto& peak : snap.value("peaks").toArray()) {
        float pk = static_cast<float>(peak.toDouble());
        viz_peaks.push_back(pk);
        peak_energy += std::abs(pk);
    }
    if (!viz_peaks.empty()) peak_energy /= static_cast<float>(viz_peaks.size());
    if (!playing && viz_widget && viz_widget->isVisible()) viz_widget->update();
    if (hero_banner) hero_banner->tickDynamic(playing, peak_energy);

    // Always check if the cover image or playing track changed on this tick so Dynamic Color updates immediately!
    const auto p_title = snap.value("player_title").toString();
    const auto sel_title = snap.value("selected_title").toString();
    const bool has_playing = !p_title.isEmpty() && p_title != "没有正在播放" && p_title != "Nothing Playing";
    if (has_playing && !first_run_tip_shown) {
        first_run_tip_shown = true;
        QSettings("mineral", "mineral-gui-qt").setValue("first_run_tip_shown", true);
    }
    const bool use_sel = !has_playing && !sel_title.isEmpty();
    const QString hero_title = has_playing ? p_title : (use_sel ? sel_title : QStringLiteral("Mineral"));
    const QString hero_kind = use_sel ? snap.value("selected_kind").toString() : QStringLiteral("song");
    const QString hero_cover = (!snap.value("cover_url").toString().isEmpty())
        ? snap.value("cover_url").toString()
        : snap.value("selected_cover").toString();
    if (cover_widget) cover_widget->setCover(hero_cover, hero_title, hero_kind);

    if (!full) return;

    const bool connected = snap.value("daemon_connected").toBool();
    daemon_connected = connected;
    close_policy = snap.value("close_policy").toString();
    auto_adapt_on = false;
    locked_on = false;
    reduce_motion_on = snap.value("reduce_motion").toBool();
    touch_on = false;
    const bool mini = snap.value("mini").toBool();
    const bool immersive = !mini && snap.value("immersive").toBool();
    const auto workspace = snap.value("workspace").toString();
    const auto content_kind = snap.value("content_kind").toString();
    current_workspace = workspace;
    current_content_kind = content_kind;
    single_mode_on = false;
    const bool queue_showing = play_queue_tab_active || workspace == "queue" || content_kind == "queue";

    if (daemon_btn) {
        daemon_btn->setText(connected ? "● Online" : "○ Offline");
        daemon_btn->setToolTip(connected ? "Daemon Online (Click to reconnect)" : "Daemon Offline (Click to launch daemon)");
    }
    daemon_label->setText(connected ? "● Online" : "○ Offline");

    const auto q_str = snap.value("search_query").toString();
    if (!search_box->hasFocus() && !q_str.isEmpty() && search_box->text() != q_str) {
        QSignalBlocker b(search_box);
        search_box->setText(q_str);
    }

    if (now_playing_tab_active) {
        for (auto* btn : nav_group->buttons()) {
            if (btn->property("nav").toString() == "now_playing") {
                btn->setChecked(true);
            }
        }
    } else if (queue_showing) {
        for (auto* btn : nav_group->buttons()) {
            if (btn->property("nav").toString() == "queue") {
                btn->setChecked(true);
            }
        }
    } else {
        for (const auto& nav_val : snap.value("nav").toArray()) {
            const auto nav_obj = nav_val.toObject();
            const auto nid = nav_obj.value("key").toString();
            const bool is_cur = nav_obj.value("current").toBool();
            for (auto* btn : nav_group->buttons()) {
                if (btn->property("nav").toString() == nid) {
                    btn->setChecked(is_cur);
                }
            }
        }
    }

    const bool show_source_chips = !queue_showing && content_kind != "settings";
    filter_chips_bar->setVisible(show_source_chips);
    if (auto* src_group = qobject_cast<QButtonGroup*>(filter_chips_bar->property("src_group_ptr").value<QObject*>())) {
        const QString cur_src = snap.value("source_filter").isNull() ? QString() : snap.value("source_filter").toString();
        for (const auto& sv : snap.value("sources").toArray()) {
            const auto sobj = sv.toObject();
            const QString sid = sobj.value("id").toString();
            if (sid.isEmpty()) continue;
            bool exists = false;
            for (auto* b : src_group->buttons()) {
                if (b->property("source_id").toString() == sid) {
                    exists = true;
                    break;
                }
            }
            if (!exists && source_chips_layout) {
                QString slabel = sid;
                if (sid == "netease") slabel = "NetEase";
                else if (sid == "bilibili") slabel = "Bilibili";
                else if (sid == "am") slabel = "Apple Music";
                else slabel = sid.left(1).toUpper() + sid.mid(1);
                auto* btn = new QPushButton(slabel);
                btn->setObjectName("chip");
                btn->setCursor(Qt::PointingHandCursor);
                btn->setCheckable(true);
                btn->setProperty("source_id", sid);
                src_group->addButton(btn);
                source_chips_layout->insertWidget(std::max(0, source_chips_layout->count() - 1), btn);
                QObject::connect(btn, &QPushButton::clicked, [sid] {
                    push_action(QString("{\"action\":\"source_filter\",\"source\":\"%1\"}").arg(escape_json(sid)));
                });
            }
        }
        for (auto* b : src_group->buttons()) {
            if (b->property("source_id").toString() == cur_src) {
                b->setChecked(true);
            }
        }
    }

    const auto crumbs_arr = snap.value("crumbs").toArray();
    QStringList crumb_parts;
    for (const auto& cv : crumbs_arr) crumb_parts.append(to_english_ui(cv.toString()));
    const bool can_go_back = !mini && !immersive && (crumb_parts.size() > 1 || (content_kind == "search" && workspace != "spotlight"));
    back_btn->setVisible(can_go_back);
    if (content_back_btn) content_back_btn->setVisible(can_go_back);
    crumbs_label->setText((!mini && crumb_parts.size() > 1) ? crumb_parts.join("  ›  ") : QString());

    auto title = queue_showing ? QStringLiteral("Play Queue") : to_english_ui(snap.value("content_title").toString());
    heading->setText(title);

    search_tabs_bar->setVisible(!queue_showing && content_kind == "search");
    const auto cur_tab = snap.value("search_tab").toString().isEmpty() ? QStringLiteral("all") : snap.value("search_tab").toString();
    for (auto* b : search_tab_group->buttons()) {
        if (b->property("tab").toString() == cur_tab) b->setChecked(true);
    }
    summary_label->setText(snap.value("search_summary").toString());

    const bool show_view_tools = !queue_showing && content_kind != "settings";
    cover_tools_bar->setVisible(show_view_tools);
    const bool is_grid_view = (snap.value("cover_mode").toString() == "grid") && content_kind != "settings" && !queue_showing;
    cover_grid_btn->setChecked(is_grid_view);
    cover_list_btn->setChecked(!is_grid_view);

    QString empty = to_english_ui(snap.value("empty").toString());
    if (!connected) empty = "Playback daemon is disconnected. Click '○ Offline' in the sidebar to launch or reconnect.";
    empty_label->setText(empty);

    const auto p_sub = snap.value("player_subtitle").toString();
    const auto p_src = snap.value("player_source").toString();
    const QString hero_sub = has_playing ? p_sub : (use_sel ? snap.value("selected_subtitle").toString() : QString());
    const QString hero_album = snap.value("selected_album").toString();
    const QString hero_src = has_playing ? p_src : snap.value("selected_source").toString();

    now_title->setText(hero_title);
    now_title->setToolTip(hero_title);
    now_title->update();
    QString desc_text = hero_sub;
    if (!hero_album.isEmpty() && !desc_text.contains(hero_album)) {
        desc_text = desc_text.isEmpty() ? hero_album : (desc_text + "  ·  " + hero_album);
    }
    base_hero_desc = desc_text;
    if (!immersive && !active_lyric_line.isEmpty()) {
        desc_text = desc_text.isEmpty() ? active_lyric_line : (desc_text + "\n" + active_lyric_line);
    } else if (desc_text.isEmpty()) {
        if (!first_run_tip_shown) {
            desc_text = QStringLiteral("Welcome to Mineral — Click any cover or track below to start listening.");
            first_run_tip_shown = true;
            QSettings("mineral", "mineral-gui-qt").setValue("first_run_tip_shown", true);
        } else {
            desc_text = QStringLiteral("Ready to play");
        }
    }
    now_artist->setText(desc_text);

    if (stream_badge) {
        if (playing) {
            stream_badge->setText(hero_src.isEmpty() ? QStringLiteral("Playing") : hero_src.toUpper());
        } else if (has_playing) {
            stream_badge->setText(hero_src.isEmpty() ? QStringLiteral("Paused") : hero_src.toUpper());
        } else {
            stream_badge->setText(QString());
        }
    }
    const auto next_t = snap.value("next_title").toString();
    next_up_label->setText(next_t.isEmpty() ? QString() : ("Next: " + next_t));

    // Playback Mode Icon-Only Button (`Sequential`, `Repeat All`, `Repeat One`, `Shuffle`)
    const QString cur_mode = snap.value("play_mode").toString();
    if (mode_cycle_btn) {
        last_play_mode = cur_mode;
        mode_cycle_btn->setText(QString());
        mode_cycle_btn->setIcon(mineral_ui::md3_icon(play_mode_icon(cur_mode), QColor("#FFFFFF"), 16));
        mode_cycle_btn->setIconSize(QSize(16, 16));
        mode_cycle_btn->setToolTip("Playback Mode: " + play_mode_text(cur_mode) + " (Click to cycle)");
    }
    lyric_cycle_btn->setText(lyric_track_text(snap.value("lyric_track").toString()));

    const bool is_loved = snap.value("loved").toBool() || snap.value("selected_loved").toBool();
    love_btn->setText(QString());
    love_btn->setToolTip(is_loved ? "Loved (Click to unfavourite)" : "Toggle Favourite (L)");
    if (is_loved != last_loved_state) {
        last_loved_state = is_loved;
        love_btn->setIcon(mineral_ui::md3_icon(
            is_loved ? mineral_ui::M3Icon::HeartFilled : mineral_ui::M3Icon::HeartOutline,
            QColor("#FFFFFF"), 16));
        love_btn->setIconSize(QSize(16, 16));
    }

    const int snap_vol = snap.value("volume").toInt(80);
    if (volume_bar) volume_bar->setVolumeFromSnapshot(snap_vol);
    if (volume && !volume->isSliderDown()) volume->setValue(snap_vol);

    // Notifications: suppress Mode Center notices, and auto-dismiss all notices after 5 seconds!
    const auto notices = snap.value("notices").toArray();
    if (notices.isEmpty()) {
        notice_bar->setVisible(false);
        last_notice_id = -1;
    } else {
        const auto last_n = notices.last().toObject();
        const int nid = last_n.value("id").toInt(-1);
        const QString raw_text = last_n.value("text").toString();
        if (raw_text.contains("模式中心") || raw_text.contains("Ctrl+Shift+M") || raw_text.contains("当前模式")) {
            notice_bar->setVisible(false);
            if (nid >= 0) push_action(QString("{\"action\":\"dismiss_notice\",\"id\":%1}").arg(nid));
        } else {
            last_notice_id = nid;
            if (nid != scheduled_notice_id) {
                scheduled_notice_id = nid;
                notice_shown_ms = QDateTime::currentMSecsSinceEpoch();
                notice_label->setText(to_english_ui(raw_text));
                notice_bar->setVisible(true);
                QTimer::singleShot(5000, window, [nid] {
                    if (last_notice_id == nid) {
                        if (notice_bar) notice_bar->setVisible(false);
                        if (nid >= 0) push_action(QString("{\"action\":\"dismiss_notice\",\"id\":%1}").arg(nid));
                    }
                });
            } else if (QDateTime::currentMSecsSinceEpoch() - notice_shown_ms >= 5000) {
                notice_bar->setVisible(false);
                if (nid >= 0) push_action(QString("{\"action\":\"dismiss_notice\",\"id\":%1}").arg(nid));
            } else {
                notice_label->setText(to_english_ui(raw_text));
                notice_bar->setVisible(true);
            }
        }
    }

    if (hero_banner) hero_banner->immersive_mode = immersive;
    if (fullscreen_lyric_list) fullscreen_lyric_list->setVisible(immersive);
    if (lyric_font_dec_btn) lyric_font_dec_btn->setVisible(!mini && (immersive || now_playing_tab_active));
    if (lyric_font_inc_btn) lyric_font_inc_btn->setVisible(!mini && (immersive || now_playing_tab_active));

    // 1. Hide sidebar and bottom area FIRST before resizing window so layout minimum size shrinks immediately!
    if (sidebar) sidebar->setVisible(!mini && !immersive);
    if (top_bento_area) top_bento_area->setVisible(!mini && !immersive);
    if (selected_card) {
        selected_card->setVisible(true);
        selected_card->setMaximumWidth(16777215);
        if (immersive) {
            if (hero_h_layout_ref) {
                hero_h_layout_ref->setContentsMargins(42, 20, 42, 20);
                hero_h_layout_ref->setSpacing(36);
            }
            selected_card->setMinimumHeight(0);
            selected_card->setMaximumHeight(16777215);
            if (cover_widget) cover_widget->setFixedSize(320, 320);
            if (now_title) {
                now_title->setWordWrap(false);
                now_title->setStyleSheet(QString());
                now_title->setFixedHeight(44);
                now_title->setMaximumWidth(16777215);
            }
            if (now_artist) {
                now_artist->setMaximumWidth(580);
                now_artist->setMaximumHeight(16777215);
            }
            if (lyric_cycle_btn) lyric_cycle_btn->setVisible(true);
            if (sel_enq_btn_ref) sel_enq_btn_ref->setVisible(true);
            if (dyn_color_btn) dyn_color_btn->setVisible(true);
            if (next_up_label) next_up_label->setVisible(true);
            if (immersive_btn_ref) immersive_btn_ref->setVisible(true);
            if (viz_widget) viz_widget->setFixedHeight(40);
            if (volume_bar) volume_bar->setFixedSize(96, 24);
            mini_restore_btn->setText("Exit Fullscreen (Esc)");
            mini_restore_btn->setVisible(true);
        } else if (mini) {
            if (hero_h_layout_ref) {
                hero_h_layout_ref->setContentsMargins(22, 14, 22, 14);
                hero_h_layout_ref->setSpacing(20);
            }
            selected_card->setMinimumHeight(256);
            selected_card->setMaximumHeight(276);
            if (cover_widget) cover_widget->setFixedSize(168, 168);
            if (now_title) {
                now_title->setWordWrap(false);
                now_title->setStyleSheet("font-size: 24px; font-weight: 800; color: #FFFFFF;");
                now_title->setFixedHeight(32);
                now_title->setMaximumWidth(355);
            }
            if (now_artist) {
                now_artist->setMaximumWidth(355);
                now_artist->setMaximumHeight(38);
            }
            if (lyric_cycle_btn) lyric_cycle_btn->setVisible(false);
            if (sel_enq_btn_ref) sel_enq_btn_ref->setVisible(false);
            if (dyn_color_btn) dyn_color_btn->setVisible(false);
            if (next_up_label) next_up_label->setVisible(false);
            if (immersive_btn_ref) immersive_btn_ref->setVisible(false);
            if (viz_widget) viz_widget->setFixedHeight(26);
            if (volume_bar) volume_bar->setFixedSize(82, 24);
            mini_restore_btn->setText("Expand");
            mini_restore_btn->setVisible(true);
        } else {
            if (hero_h_layout_ref) {
                hero_h_layout_ref->setContentsMargins(42, 20, 42, 20);
                hero_h_layout_ref->setSpacing(36);
            }
            selected_card->setMinimumHeight(268);
            selected_card->setMaximumHeight(285);
            if (cover_widget) cover_widget->setFixedSize(228, 228);
            if (now_title) {
                now_title->setWordWrap(false);
                now_title->setStyleSheet(QString());
                now_title->setFixedHeight(44);
                now_title->setMaximumWidth(16777215);
            }
            if (now_artist) {
                now_artist->setMaximumWidth(580);
                now_artist->setMaximumHeight(16777215);
            }
            if (lyric_cycle_btn) lyric_cycle_btn->setVisible(true);
            if (sel_enq_btn_ref) sel_enq_btn_ref->setVisible(true);
            if (dyn_color_btn) dyn_color_btn->setVisible(true);
            if (next_up_label) next_up_label->setVisible(true);
            if (immersive_btn_ref) immersive_btn_ref->setVisible(true);
            if (viz_widget) viz_widget->setFixedHeight(40);
            if (volume_bar) volume_bar->setFixedSize(96, 24);
            mini_restore_btn->setText("Expand");
            mini_restore_btn->setVisible(false);
        }
    }

    // 2. Activate layouts when transitioning between Mini Player and Normal Player
    if (mini != was_mini) {
        if (selected_card && selected_card->layout()) selected_card->layout()->activate();
        if (main_col_ref && main_col_ref->layout()) main_col_ref->layout()->activate();
        if (window->centralWidget() && window->centralWidget()->layout()) {
            window->centralWidget()->layout()->activate();
        }
    }

    // 3. Resize window to compact Mini Player dimensions (650x268, fixed & non-fullscreenable) or restore previous normal window size & position
    if (mini && !was_mini) {
        was_maximized_before_mini = window->isMaximized();
        if (!window->isFullScreen() && !window->isMaximized() && window->width() >= 860 && window->height() >= 560) {
            normal_win_size = window->size();
            normal_win_pos = window->pos();
            normal_pos_valid = true;
            normal_win_geom = window->saveGeometry();
        }
        suppress_normal_geom_save_until_ms = QDateTime::currentMSecsSinceEpoch() + 500;
        was_mini = true;
        if (window->isFullScreen() || window->isMaximized()) {
            window->setWindowState(window->windowState() & ~(Qt::WindowFullScreen | Qt::WindowMaximized));
            window->showNormal();
        }
        window->setWindowFlag(Qt::WindowMaximizeButtonHint, false);
        window->setWindowFlag(Qt::WindowFullscreenButtonHint, false);
        window->setFixedSize(650, 268);
        window->show();
        QTimer::singleShot(15, window, [] {
            if (was_mini && window) {
                if (window->isFullScreen() || window->isMaximized()) {
                    window->setWindowState(window->windowState() & ~(Qt::WindowFullScreen | Qt::WindowMaximized));
                    window->showNormal();
                }
                window->setFixedSize(650, 268);
            }
        });
    } else if (!mini && was_mini) {
        suppress_normal_geom_save_until_ms = QDateTime::currentMSecsSinceEpoch() + 600;
        const QSize target_sz = (normal_win_size.width() >= 860 && normal_win_size.height() >= 560)
            ? normal_win_size
            : QSize(1380, 860);
        const QPoint target_pos = normal_win_pos;
        const bool has_pos = normal_pos_valid;
        const QByteArray saved_geom = normal_win_geom;
        const bool restore_max = was_maximized_before_mini;

        was_mini = false;
        window->setWindowFlag(Qt::WindowMaximizeButtonHint, true);
        window->setWindowFlag(Qt::WindowFullscreenButtonHint, true);
        window->setMaximumSize(16777215, 16777215);
        window->setMinimumSize(940, 580);
        window->show();
        if (!immersive) {
            if (restore_max) {
                window->showMaximized();
            } else {
                if (!saved_geom.isEmpty()) {
                    window->restoreGeometry(saved_geom);
                }
                window->resize(target_sz);
                if (has_pos) {
                    window->move(target_pos);
                }
                QTimer::singleShot(25, window, [target_sz, target_pos, has_pos, saved_geom] {
                    if (!was_mini && window && !window->isFullScreen() && !window->isMaximized()) {
                        if (!saved_geom.isEmpty()) {
                            window->restoreGeometry(saved_geom);
                        }
                        window->resize(target_sz);
                        if (has_pos) {
                            window->move(target_pos);
                        }
                    }
                });
            }
        }
    }

    // Sidebar "Now Playing" tab: replace song list (`center_panel`) with synced lyrics (`lyrics_card`) and hide the 2x2 cover grid (`bottom_bento_area`)
    if (center_panel) center_panel->setVisible(!now_playing_tab_active);
    if (bottom_bento_area) bottom_bento_area->setVisible(!now_playing_tab_active);
    if (lyrics_card) lyrics_card->setVisible(now_playing_tab_active);
    if (immersive || now_playing_tab_active) {
        const int cur_r = lyric_list ? lyric_list->property("active_row").toInt() : -1;
        if (cur_r >= 0) smooth_scroll_lyrics(cur_r);
    }

    close_box->setVisible(snap.value("close_prompt").toBool());
    queue_tools->setVisible(!mini && queue_showing);

    if (window->property("touch_styled").toBool() != touch_on) {
        window->setProperty("touch_styled", touch_on);
        apply_m3_theme();
    }
    window->setWindowTitle(!has_playing ? "Mineral" : (p_title + " — Mineral"));

    // Mode Center is disabled; never show mode_dialog
    if (mode_dialog && mode_dialog->isVisible()) mode_dialog->hide();

    const bool palette_open = snap.value("palette_open").toBool();
    const bool help_open = snap.value("help_open").toBool();
    if (palette_open) {
        QString pal_sig;
        for (const auto& command : snap.value("commands").toArray()) {
            const auto item_obj = command.toObject();
            pal_sig += item_obj.value("id").toString() + "\n";
        }
        if (palette_list->property("sig").toString() != pal_sig) {
            palette_list->setProperty("sig", pal_sig);
            palette_list->clear();
            for (const auto& command : snap.value("commands").toArray()) {
                const auto item_obj = command.toObject();
                auto* item = new QListWidgetItem(item_obj.value("title").toString());
                item->setData(Qt::UserRole, item_obj.value("id").toString());
                item->setSizeHint(QSize(0, 40));
                palette_list->addItem(item);
            }
        }
        if (!palette_dialog->isVisible()) {
            palette_dialog->show();
            palette_dialog->raise();
            palette_dialog->activateWindow();
            if (palette_search_box) palette_search_box->setFocus();
        }
    } else if (palette_dialog->isVisible()) palette_dialog->hide();

    if (help_open) {
        if (!help_dialog->isVisible()) {
            help_dialog->show();
            help_dialog->raise();
        }
    } else if (help_dialog && help_dialog->isVisible()) {
        help_dialog->hide();
    }

    QJsonArray content = snap.value("rows").toArray();
    if (workspace == "spotlight" && content.isEmpty() && q_str.isEmpty()) {
        for (const auto& rs : snap.value("recent_searches").toArray()) {
            QJsonObject o;
            o["key"] = "recent_search:" + rs.toString();
            o["kind"] = "search_history";
            o["title"] = "Recent search: " + rs.toString();
            o["subtitle"] = "Click to search again";
            content.append(o);
        }
        for (const auto& rp : snap.value("recent_plays").toArray()) {
            content.append(rp);
        }
    } else if (queue_showing && workspace != "queue" && content_kind != "queue") {
        content = snap.value("queue").toArray();
    }
    const bool is_grid = ((content_kind == "grid") || (snap.value("cover_mode").toString() == "grid")) && content_kind != "settings" && !queue_showing;
    if (col_header_bar) col_header_bar->setVisible(!is_grid && content_kind != "settings" && !content.isEmpty());
    if (stats_label) {
        const qint64 total_ms = snap.value("total_duration_ms").toVariant().toLongLong();
        const qint64 total_min = total_ms / 60000;
        if (content.isEmpty()) {
            stats_label->setText("0 items");
        } else if (total_min > 0) {
            stats_label->setText(QString("%1 items  ·  %2m").arg(content.size()).arg(total_min));
        } else {
            stats_label->setText(QString("%1 items").arg(content.size()));
        }
    }
    fill_list(rows, content, snap.value("selected_key").toString(), is_grid);
    const bool has_rows = !content.isEmpty();
    rows->setVisible(has_rows);
    empty_label->setVisible(!has_rows && !empty.isEmpty());

    // Populate Right-Column "New Albums" 2x2 Cover Grid (`lib_browser_list` - first 4 albums from current song's genre search)
    QJsonArray grid_cards;
    QStringList seen_keys;
    auto push_card = [&](const QJsonValue& val) {
        if (grid_cards.size() >= 4) return;
        const auto obj = val.toObject();
        const auto k = obj.value("key").toString();
        const auto kd = obj.value("kind").toString();
        if (k.isEmpty() || k.startsWith("settings:") || kd == "source" || seen_keys.contains(k)) return;
        seen_keys.append(k);
        grid_cards.append(obj);
    };
    for (const auto& v : snap.value("rec_albums").toArray()) push_card(v);
    for (const auto& v : snap.value("search_albums").toArray()) push_card(v);
    for (const auto& v : snap.value("library_items").toArray()) {
        const auto kd = v.toObject().value("kind").toString();
        if (kd == "album") push_card(v);
    }
    for (const auto& v : snap.value("library_items").toArray()) {
        const auto kd = v.toObject().value("kind").toString();
        if (kd == "playlist") push_card(v);
    }
    for (const auto& v : content) {
        const auto kd = v.toObject().value("kind").toString();
        if (kd == "album" || kd == "playlist") push_card(v);
    }
    fill_list(lib_browser_list, grid_cards, snap.value("selected_key").toString(), true);

    if (immersive && !window->isFullScreen()) window->showFullScreen();
    else if (!immersive && window->isFullScreen()) window->showNormal();
}

extern "C" int qt_poll_action(char* buf, int cap) {
    if (actions.empty() || cap <= 1) return 0;
    const auto next = actions.front();
    actions.pop_front();
    const auto size = static_cast<int>(std::min(next.size(), static_cast<size_t>(cap - 1)));
    std::memcpy(buf, next.data(), static_cast<size_t>(size));
    buf[size] = '\0';
    return size;
}

extern "C" int qt_frame_interval_us() {
    const double hz = current_screen_refresh_hz(window);
    return std::clamp(static_cast<int>(std::round(1000000.0 / hz)), 1000, 16667);
}
