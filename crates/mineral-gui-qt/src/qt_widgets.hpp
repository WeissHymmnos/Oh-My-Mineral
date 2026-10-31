#pragma once

#include <QAbstractItemView>
#include <QButtonGroup>
#include <QColor>
#include <QDir>
#include <QElapsedTimer>
#include <QFileInfo>
#include <QFont>
#include <QFontMetrics>
#include <QHash>
#include <QIcon>
#include <QImage>
#include <QImageReader>
#include <QListWidget>
#include <QMouseEvent>
#include <QPainter>
#include <QPainterPath>
#include <QPixmap>
#include <QPushButton>
#include <QStyledItemDelegate>
#include <QWheelEvent>
#include <QWidget>
#include <algorithm>
#include <cmath>
#include <functional>
#include <vector>

namespace mineral_ui {

enum RowRoles {
    RoleKey = Qt::UserRole,
    RoleTitle = Qt::UserRole + 1,
    RoleSubtitle = Qt::UserRole + 2,
    RoleAlbum = Qt::UserRole + 3,
    RoleSource = Qt::UserRole + 4,
    RoleDuration = Qt::UserRole + 5,
    RoleKind = Qt::UserRole + 6,
    RoleCurrent = Qt::UserRole + 7,
    RoleLoved = Qt::UserRole + 8,
    RoleCover = Qt::UserRole + 9,
    RoleGridMode = Qt::UserRole + 10,
    RoleIndex = Qt::UserRole + 11
};

// ============================================================================
// Reference Palette: Warm Coral-Terracotta Sidebar + Almond Hero + Pure White
// ============================================================================
struct M3ColorScheme {
    QString preset_id{"coral"};
    QColor seed{"#EA957A"};

    // Coral / Primary Accents
    QColor primary{"#EA957A"};            // Coral sidebar & play button
    QColor primaryDark{"#E07A5F"};        // Active waveform & highlights
    QColor onPrimary{"#FFFFFF"};
    QColor primaryContainer{"#F2AE98"};   // Sidebar search pill
    QColor onPrimaryContainer{"#FFFFFF"};

    // Hero Banner & Pill Tags
    QColor heroBgLeft{"#DFCEC4"};         // Warm almond-taupe left
    QColor heroBgRight{"#EFE6E0"};        // Soft cream right
    QColor heroTagBg{"#DFB5A6"};          // "Indie Dance" / "House" blush pill
    QColor heroTagFg{"#FFFFFF"};
    QColor waveActive{"#E07A5F"};         // Played waveform bars
    QColor waveInactive{"#C8B8AF"};       // Unplayed waveform bars

    // Secondary / Tonal
    QColor secondary{"#E07A5F"};
    QColor onSecondary{"#FFFFFF"};
    QColor secondaryContainer{"#F7EAE5"};
    QColor onSecondaryContainer{"#5C3B31"};

    // Tertiary
    QColor tertiary{"#D48C72"};
    QColor onTertiary{"#FFFFFF"};
    QColor tertiaryContainer{"#F5E3DC"};
    QColor onTertiaryContainer{"#4A2C21"};

    // Neutral Surfaces (Pure White Content Area)
    QColor surfaceDim{"#F6F3F1"};
    QColor surface{"#FFFFFF"};
    QColor surfaceBright{"#FFFFFF"};
    QColor surfaceContainerLowest{"#FFFFFF"};
    QColor surfaceContainerLow{"#FAF8F7"};
    QColor surfaceContainer{"#F5F1EE"};
    QColor surfaceContainerHigh{"#EFEAE6"};
    QColor surfaceContainerHighest{"#E6DFDA"};

    // Typography & Dividers
    QColor onSurface{"#2B2625"};          // Bold dark charcoal-espresso
    QColor onSurfaceVariant{"#6E6662"};   // Artist / Album text
    QColor mutedText{"#B5AFA9"};          // Column headers & duration
    QColor outline{"#DCD5D0"};
    QColor outlineVariant{"#F0ECE9"};

    static QColor toneColor(int hue, int sat255, int light255) {
        return QColor::fromHsl((hue % 360 + 360) % 360, std::clamp(sat255, 0, 255), std::clamp(light255, 0, 255));
    }

    static M3ColorScheme fromPreset(const QString& id = "coral") {
        M3ColorScheme sc;
        sc.preset_id = id;
        if (id == "sage") {
            sc.primary = QColor("#7FA693");
            sc.primaryDark = QColor("#5E8C76");
            sc.primaryContainer = QColor("#9BBBAE");
            sc.heroBgLeft = QColor("#CFDAD4");
            sc.heroBgRight = QColor("#E6ECE8");
            sc.heroTagBg = QColor("#A9C2B5");
            sc.waveActive = QColor("#5E8C76");
            sc.waveInactive = QColor("#B5C2BB");
            sc.secondaryContainer = QColor("#EAF1ED");
            sc.onSecondaryContainer = QColor("#2F473C");
        } else if (id == "lavender") {
            sc.primary = QColor("#9B89B3");
            sc.primaryDark = QColor("#7E6B99");
            sc.primaryContainer = QColor("#B3A3C8");
            sc.heroBgLeft = QColor("#D8D1E0");
            sc.heroBgRight = QColor("#ECE8F0");
            sc.heroTagBg = QColor("#BEB1CE");
            sc.waveActive = QColor("#7E6B99");
            sc.waveInactive = QColor("#C0B8CA");
            sc.secondaryContainer = QColor("#EFEBF5");
            sc.onSecondaryContainer = QColor("#3D314F");
        } else if (id == "ocean") {
            sc.primary = QColor("#789EB8");
            sc.primaryDark = QColor("#5882A0");
            sc.primaryContainer = QColor("#96B6CC");
            sc.heroBgLeft = QColor("#CED9E0");
            sc.heroBgRight = QColor("#E6ECF0");
            sc.heroTagBg = QColor("#ABC1D1");
            sc.waveActive = QColor("#5882A0");
            sc.waveInactive = QColor("#B4C0C8");
            sc.secondaryContainer = QColor("#EBF1F5");
            sc.onSecondaryContainer = QColor("#2B4252");
        }
        return sc; // default "coral" is exact match to reference image
    }

    static M3ColorScheme fromSeed(const QColor& s, const QColor& s2 = QColor(), const QColor& s3 = QColor()) {
        if (!s.isValid()) return fromPreset("coral");
        M3ColorScheme sc;
        sc.preset_id = QStringLiteral("dynamic");
        sc.seed = s;

        int h1 = s.hslHue();
        int sat1 = s.hslSaturation();
        if (h1 < 0 || sat1 < 18) {
            // Monochrome / Black & White cover: Editorial Slate-Warm Monochrome palette
            int h_mono = (h1 >= 0) ? h1 : 210;
            sc.primary = toneColor(h_mono, 28, 122);
            sc.primaryDark = toneColor(h_mono, 36, 92);
            sc.primaryContainer = toneColor(h_mono, 24, 152);
            sc.heroBgLeft = toneColor(h_mono, 20, 178);
            sc.heroBgRight = toneColor(h_mono, 16, 208);
            sc.heroTagBg = toneColor(h_mono, 26, 148);
            sc.waveActive = sc.primaryDark;
            sc.waveInactive = toneColor(h_mono, 14, 162);
            sc.secondary = sc.primaryDark;
            sc.secondaryContainer = toneColor(h_mono, 22, 232);
            sc.onSecondaryContainer = toneColor(h_mono, 42, 52);
            sc.tertiary = toneColor((h_mono + 30) % 360, 32, 130);
            return sc;
        }

        int h2 = (s2.isValid() && s2.hslHue() >= 0) ? s2.hslHue() : ((h1 + 28) % 360);
        int h3 = (s3.isValid() && s3.hslHue() >= 0) ? s3.hslHue() : ((h1 + 325) % 360);
        int eff_sat = std::clamp(sat1, 78, 162);

        // Editorial Material 3 Tonal Mapping from Cover Hue
        sc.primary = toneColor(h1, eff_sat, 148);
        sc.primaryDark = toneColor(h1, std::clamp(eff_sat + 24, 95, 190), 116);
        sc.primaryContainer = toneColor(h1, std::clamp(eff_sat - 10, 65, 145), 174);

        sc.heroBgLeft = toneColor(h1, std::clamp(eff_sat * 62 / 100, 48, 112), 188);
        sc.heroBgRight = toneColor(h2, std::clamp(eff_sat * 52 / 100, 38, 98), 212);
        sc.heroTagBg = toneColor(h1, std::clamp(eff_sat * 68 / 100, 56, 128), 162);
        sc.waveActive = sc.primaryDark;
        sc.waveInactive = toneColor(h1, 36, 168);

        sc.secondary = sc.primaryDark;
        sc.secondaryContainer = toneColor(h1, 64, 235);
        sc.onSecondaryContainer = toneColor(h1, 115, 58);

        sc.tertiary = toneColor(h3, std::clamp(eff_sat, 75, 155), 142);
        sc.tertiaryContainer = toneColor(h3, 58, 228);
        sc.onTertiaryContainer = toneColor(h3, 105, 54);
        return sc;
    }
};

inline QString current_m3_preset = QStringLiteral("dynamic");
inline bool dynamic_cover_color = true;
inline M3ColorScheme m3 = M3ColorScheme::fromPreset("coral");
inline std::function<void()> on_m3_theme_changed;

inline void extract_palette_from_image(const QImage& img, QColor* out_prim, QColor* out_sec, QColor* out_tert) {
    if (img.isNull()) {
        if (out_prim) *out_prim = QColor("#EA957A");
        if (out_sec)  *out_sec  = QColor("#DFCEC4");
        if (out_tert) *out_tert = QColor("#E07A5F");
        return;
    }
    QImage small = img.scaled(36, 36, Qt::IgnoreAspectRatio, Qt::SmoothTransformation);
    struct HueBin {
        long long weight = 0;
        long long r = 0, g = 0, b = 0, count = 0;
    };
    HueBin bins[12];
    long long gray_r = 0, gray_g = 0, gray_b = 0, gray_n = 0;

    for (int y = 0; y < small.height(); ++y) {
        for (int x = 0; x < small.width(); ++x) {
            const QColor c = small.pixelColor(x, y);
            const int sat = c.hsvSaturation();
            const int val = c.value();
            const int hue = c.hsvHue();
            if (val > 25 && val < 245) {
                gray_r += c.red();
                gray_g += c.green();
                gray_b += c.blue();
                ++gray_n;
            }
            // Chroma-weighted clustering: ignores pure black/white borders so colored art (like Mozart cyan/gold) stands out!
            if (hue >= 0 && sat >= 28 && val >= 32 && !(val > 242 && sat < 42)) {
                const int bin = (hue / 30) % 12;
                const long long w = static_cast<long long>(sat) * sat * (36 + std::min(val, 255 - std::abs(c.lightness() - 128))) / 800 + 1;
                bins[bin].weight += w;
                bins[bin].r += c.red() * w;
                bins[bin].g += c.green() * w;
                bins[bin].b += c.blue() * w;
                bins[bin].count += w;
            }
        }
    }

    int best1 = -1, best2 = -1, best3 = -1;
    long long max_w1 = 0, max_w2 = 0, max_w3 = 0;
    for (int i = 0; i < 12; ++i) {
        if (bins[i].weight > max_w1) {
            max_w1 = bins[i].weight;
            best1 = i;
        }
    }
    for (int i = 0; i < 12; ++i) {
        if (i == best1) continue;
        if (bins[i].weight > max_w2) {
            max_w2 = bins[i].weight;
            best2 = i;
        }
    }
    for (int i = 0; i < 12; ++i) {
        if (i == best1 || i == best2) continue;
        if (bins[i].weight > max_w3) {
            max_w3 = bins[i].weight;
            best3 = i;
        }
    }

    auto bin_color = [&](int idx) -> QColor {
        if (idx >= 0 && bins[idx].count > 0) {
            return QColor(bins[idx].r / bins[idx].count,
                          bins[idx].g / bins[idx].count,
                          bins[idx].b / bins[idx].count);
        }
        if (gray_n > 0) {
            return QColor(gray_r / gray_n, gray_g / gray_n, gray_b / gray_n);
        }
        return QColor("#EA957A");
    };

    QColor c1 = bin_color(best1);
    QColor c2 = (best2 >= 0 && max_w2 > max_w1 / 12)
        ? bin_color(best2)
        : M3ColorScheme::toneColor((std::max(0, c1.hslHue()) + 32) % 360, std::clamp(c1.hslSaturation(), 60, 150), 175);
    QColor c3 = (best3 >= 0 && max_w3 > max_w1 / 16)
        ? bin_color(best3)
        : M3ColorScheme::toneColor((std::max(0, c1.hslHue()) + 325) % 360, std::clamp(c1.hslSaturation() + 20, 75, 175), 145);

    if (out_prim) *out_prim = c1;
    if (out_sec)  *out_sec  = c2;
    if (out_tert) *out_tert = c3;
}

inline QColor extract_seed_from_image(const QImage& img) {
    QColor c1;
    extract_palette_from_image(img, &c1, nullptr, nullptr);
    return c1;
}

// ============================================================================
// Vector Icon Renderer (24dp Clean Editorial & Material Icons)
// ============================================================================
enum class M3Icon {
    Play,
    Pause,
    Prev,
    Next,
    HeartFilled,
    HeartOutline,
    Repeat,
    Queue,
    Lyrics,
    Equalizer,
    Search,
    Library,
    Playlists,
    Downloads,
    Settings,
    Palette,
    Back,
    Volume,
    Fullscreen,
    AddQueue,
    Help,
    MoreVert,
    Check,
    SidebarCollapse,
    SidebarExpand,
    ModeSequential,
    ModeRepeatAll,
    ModeRepeatOne,
    ModeShuffle,
    MiniPlayer
};

inline void paint_md3_icon(QPainter* p, M3Icon icon, const QRectF& r, const QColor& color) {
    p->save();
    p->setRenderHint(QPainter::Antialiasing, true);
    const qreal s = std::min(r.width(), r.height());
    p->translate(r.left() + (r.width() - s) / 2.0, r.top() + (r.height() - s) / 2.0);
    p->scale(s / 24.0, s / 24.0);

    QPen pen(color, 2.0, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin);
    p->setPen(Qt::NoPen);
    p->setBrush(color);

    switch (icon) {
        case M3Icon::Play: {
            QPainterPath path;
            path.moveTo(8.5, 5.5);
            path.lineTo(19.0, 12.0);
            path.lineTo(8.5, 18.5);
            path.closeSubpath();
            p->drawPath(path);
            break;
        }
        case M3Icon::Pause: {
            p->drawRoundedRect(QRectF(6.5, 5.0, 4.0, 14.0), 1.5, 1.5);
            p->drawRoundedRect(QRectF(13.5, 5.0, 4.0, 14.0), 1.5, 1.5);
            break;
        }
        case M3Icon::Prev: {
            p->drawRoundedRect(QRectF(6.0, 6.0, 2.4, 12.0), 1.0, 1.0);
            QPainterPath path;
            path.moveTo(18.0, 6.0);
            path.lineTo(9.5, 12.0);
            path.lineTo(18.0, 18.0);
            path.closeSubpath();
            p->drawPath(path);
            break;
        }
        case M3Icon::Next: {
            p->drawRoundedRect(QRectF(15.6, 6.0, 2.4, 12.0), 1.0, 1.0);
            QPainterPath path;
            path.moveTo(6.0, 6.0);
            path.lineTo(14.5, 12.0);
            path.lineTo(6.0, 18.0);
            path.closeSubpath();
            p->drawPath(path);
            break;
        }
        case M3Icon::HeartFilled:
        case M3Icon::HeartOutline: {
            QPainterPath path;
            path.moveTo(12.0, 20.2);
            path.cubicTo(6.5, 15.4, 3.0, 12.3, 3.0, 8.5);
            path.cubicTo(3.0, 5.7, 5.2, 3.5, 8.0, 3.5);
            path.cubicTo(9.7, 3.5, 11.2, 4.3, 12.0, 5.6);
            path.cubicTo(12.8, 4.3, 14.3, 3.5, 16.0, 3.5);
            path.cubicTo(18.8, 3.5, 21.0, 5.7, 21.0, 8.5);
            path.cubicTo(21.0, 12.3, 17.5, 15.4, 12.0, 20.2);
            if (icon == M3Icon::HeartFilled) {
                p->drawPath(path);
            } else {
                p->setBrush(Qt::NoBrush);
                p->setPen(pen);
                p->drawPath(path);
            }
            break;
        }
        case M3Icon::Repeat: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(4.5, 6.5, 15.0, 11.0), 4.0, 4.0);
            p->drawLine(QPointF(15.5, 4.2), QPointF(18.0, 6.5));
            p->drawLine(QPointF(15.5, 8.8), QPointF(18.0, 6.5));
            break;
        }
        case M3Icon::Queue: {
            p->drawRoundedRect(QRectF(3.5, 6.0, 11.5, 2.2), 1.1, 1.1);
            p->drawRoundedRect(QRectF(3.5, 11.0, 11.5, 2.2), 1.1, 1.1);
            p->drawRoundedRect(QRectF(3.5, 16.0, 7.5, 2.2), 1.1, 1.1);
            p->drawEllipse(QPointF(16.5, 17.0), 2.6, 2.6);
            p->drawRoundedRect(QRectF(17.9, 8.0, 2.0, 9.0), 1.0, 1.0);
            p->drawRoundedRect(QRectF(17.9, 8.0, 3.6, 2.2), 1.0, 1.0);
            break;
        }
        case M3Icon::Lyrics: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(4.0, 4.5, 16.0, 13.0), 3.5, 3.5);
            p->drawLine(QPointF(8.0, 9.0), QPointF(16.0, 9.0));
            p->drawLine(QPointF(8.0, 13.0), QPointF(13.5, 13.0));
            break;
        }
        case M3Icon::Equalizer: {
            p->drawRoundedRect(QRectF(4.5, 10.0, 3.0, 9.0), 1.5, 1.5);
            p->drawRoundedRect(QRectF(10.5, 5.0, 3.0, 14.0), 1.5, 1.5);
            p->drawRoundedRect(QRectF(16.5, 12.0, 3.0, 7.0), 1.5, 1.5);
            break;
        }
        case M3Icon::Search: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawEllipse(QPointF(10.5, 10.5), 5.8, 5.8);
            p->drawLine(QPointF(15.0, 15.0), QPointF(20.0, 20.0));
            break;
        }
        case M3Icon::Library: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(3.5, 5.0, 13.0, 10.0), 2.0, 2.0);
            p->drawRoundedRect(QRectF(8.5, 9.5, 12.0, 9.5), 2.0, 2.0);
            break;
        }
        case M3Icon::Playlists: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawArc(QRectF(4.5, 5.0, 15.0, 14.0), 0 * 16, 180 * 16);
            p->setBrush(color);
            p->drawRoundedRect(QRectF(3.5, 11.5, 3.5, 6.5), 1.5, 1.5);
            p->drawRoundedRect(QRectF(17.0, 11.5, 3.5, 6.5), 1.5, 1.5);
            break;
        }
        case M3Icon::Downloads: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(3.5, 6.5, 17.0, 12.0), 2.0, 2.0);
            p->drawLine(QPointF(3.5, 9.5), QPointF(10.0, 9.5));
            break;
        }
        case M3Icon::Settings: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(4.0, 7.0), QPointF(20.0, 7.0));
            p->drawLine(QPointF(4.0, 17.0), QPointF(20.0, 17.0));
            p->setBrush(color);
            p->drawEllipse(QPointF(9.0, 7.0), 2.4, 2.4);
            p->drawEllipse(QPointF(15.0, 17.0), 2.4, 2.4);
            break;
        }
        case M3Icon::Palette: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(4.0, 4.0, 7.0, 7.0), 2.0, 2.0);
            p->drawRoundedRect(QRectF(13.0, 4.0, 7.0, 7.0), 2.0, 2.0);
            p->drawRoundedRect(QRectF(4.0, 13.0, 7.0, 7.0), 2.0, 2.0);
            p->drawRoundedRect(QRectF(13.0, 13.0, 7.0, 7.0), 2.0, 2.0);
            break;
        }
        case M3Icon::Back: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(19.0, 12.0), QPointF(5.0, 12.0));
            p->drawLine(QPointF(11.0, 6.0), QPointF(5.0, 12.0));
            p->drawLine(QPointF(11.0, 18.0), QPointF(5.0, 12.0));
            break;
        }
        case M3Icon::Volume: {
            QPainterPath spk;
            spk.moveTo(4.0, 9.5);
            spk.lineTo(8.0, 9.5);
            spk.lineTo(13.0, 5.5);
            spk.lineTo(13.0, 18.5);
            spk.lineTo(8.0, 14.5);
            spk.lineTo(4.0, 14.5);
            spk.closeSubpath();
            p->drawPath(spk);
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawArc(QRectF(12.0, 7.5, 6.5, 9.0), -60 * 16, 120 * 16);
            break;
        }
        case M3Icon::Fullscreen: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(4.5, 9.0), QPointF(4.5, 4.5));
            p->drawLine(QPointF(4.5, 4.5), QPointF(9.0, 4.5));
            p->drawLine(QPointF(19.5, 9.0), QPointF(19.5, 4.5));
            p->drawLine(QPointF(19.5, 4.5), QPointF(15.0, 4.5));
            p->drawLine(QPointF(4.5, 15.0), QPointF(4.5, 19.5));
            p->drawLine(QPointF(4.5, 19.5), QPointF(9.0, 19.5));
            p->drawLine(QPointF(19.5, 15.0), QPointF(19.5, 19.5));
            p->drawLine(QPointF(19.5, 19.5), QPointF(15.0, 19.5));
            break;
        }
        case M3Icon::AddQueue: {
            p->drawRoundedRect(QRectF(3.5, 6.0, 11.0, 2.2), 1.1, 1.1);
            p->drawRoundedRect(QRectF(3.5, 11.0, 11.0, 2.2), 1.1, 1.1);
            p->drawRoundedRect(QRectF(3.5, 16.0, 7.0, 2.2), 1.1, 1.1);
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(17.0, 11.5), QPointF(17.0, 18.5));
            p->drawLine(QPointF(13.5, 15.0), QPointF(20.5, 15.0));
            break;
        }
        case M3Icon::Help: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawEllipse(QPointF(12.0, 12.0), 8.0, 8.0);
            p->drawArc(QRectF(9.5, 7.5, 5.0, 4.5), 0 * 16, 180 * 16);
            p->drawLine(QPointF(12.0, 12.0), QPointF(12.0, 14.0));
            p->setBrush(color);
            p->setPen(Qt::NoPen);
            p->drawEllipse(QPointF(12.0, 16.8), 1.2, 1.2);
            break;
        }
        case M3Icon::MoreVert: {
            p->drawEllipse(QPointF(12.0, 6.0), 1.8, 1.8);
            p->drawEllipse(QPointF(12.0, 12.0), 1.8, 1.8);
            p->drawEllipse(QPointF(12.0, 18.0), 1.8, 1.8);
            break;
        }
        case M3Icon::Check: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(5.0, 12.5), QPointF(9.8, 17.2));
            p->drawLine(QPointF(9.8, 17.2), QPointF(19.0, 7.5));
            break;
        }
        case M3Icon::SidebarCollapse: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(13.0, 6.5), QPointF(7.5, 12.0));
            p->drawLine(QPointF(7.5, 12.0), QPointF(13.0, 17.5));
            p->drawLine(QPointF(18.0, 6.5), QPointF(12.5, 12.0));
            p->drawLine(QPointF(12.5, 12.0), QPointF(18.0, 17.5));
            break;
        }
        case M3Icon::SidebarExpand: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(6.5, 6.5), QPointF(12.0, 12.0));
            p->drawLine(QPointF(12.0, 12.0), QPointF(6.5, 17.5));
            p->drawLine(QPointF(11.5, 6.5), QPointF(17.0, 12.0));
            p->drawLine(QPointF(17.0, 12.0), QPointF(11.5, 17.5));
            break;
        }
        case M3Icon::ModeSequential: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(4.0, 8.0), QPointF(19.0, 8.0));
            p->drawLine(QPointF(15.5, 5.0), QPointF(19.0, 8.0));
            p->drawLine(QPointF(15.5, 11.0), QPointF(19.0, 8.0));
            p->drawLine(QPointF(4.0, 16.0), QPointF(19.0, 16.0));
            p->drawLine(QPointF(15.5, 13.0), QPointF(19.0, 16.0));
            p->drawLine(QPointF(15.5, 19.0), QPointF(19.0, 16.0));
            break;
        }
        case M3Icon::ModeRepeatAll: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(4.0, 6.5, 16.0, 11.0), 4.2, 4.2);
            p->drawLine(QPointF(15.2, 4.0), QPointF(18.0, 6.5));
            p->drawLine(QPointF(15.2, 9.0), QPointF(18.0, 6.5));
            p->drawLine(QPointF(8.8, 15.0), QPointF(6.0, 17.5));
            p->drawLine(QPointF(8.8, 20.0), QPointF(6.0, 17.5));
            break;
        }
        case M3Icon::ModeRepeatOne: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(4.0, 6.5, 16.0, 11.0), 4.2, 4.2);
            p->drawLine(QPointF(15.2, 4.0), QPointF(18.0, 6.5));
            p->drawLine(QPointF(15.2, 9.0), QPointF(18.0, 6.5));
            p->drawLine(QPointF(12.2, 9.5), QPointF(12.2, 14.8));
            p->drawLine(QPointF(10.5, 10.8), QPointF(12.2, 9.5));
            break;
        }
        case M3Icon::ModeShuffle: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawLine(QPointF(4.0, 7.0), QPointF(9.0, 7.0));
            p->drawLine(QPointF(9.0, 7.0), QPointF(15.0, 17.0));
            p->drawLine(QPointF(15.0, 17.0), QPointF(20.0, 17.0));
            p->drawLine(QPointF(17.0, 14.5), QPointF(20.0, 17.0));
            p->drawLine(QPointF(17.0, 19.5), QPointF(20.0, 17.0));

            p->drawLine(QPointF(4.0, 17.0), QPointF(9.0, 17.0));
            p->drawLine(QPointF(9.0, 17.0), QPointF(15.0, 7.0));
            p->drawLine(QPointF(15.0, 7.0), QPointF(20.0, 7.0));
            p->drawLine(QPointF(17.0, 4.5), QPointF(20.0, 7.0));
            p->drawLine(QPointF(17.0, 9.5), QPointF(20.0, 7.0));
            break;
        }
        case M3Icon::MiniPlayer: {
            p->setBrush(Qt::NoBrush);
            p->setPen(pen);
            p->drawRoundedRect(QRectF(3.5, 5.0, 17.0, 14.0), 2.5, 2.5);
            p->setPen(Qt::NoPen);
            p->setBrush(color);
            p->drawRoundedRect(QRectF(11.0, 11.2, 7.2, 5.6), 1.4, 1.4);
            break;
        }
    }
    p->restore();
}

inline QIcon md3_icon(M3Icon icon, const QColor& color, int size = 20) {
    QPixmap pm(size * 2, size * 2);
    pm.setDevicePixelRatio(2.0);
    pm.fill(Qt::transparent);
    QPainter p(&pm);
    paint_md3_icon(&p, icon, QRectF(0, 0, size, size), color);
    return QIcon(pm);
}

// ============================================================================
// Left Coral Sidebar Navigation Item (`Discover`, `Favourites`, `Playlists`...)
// Supports both Expanded (Icon + English Text) and Collapsed (Icon-Only) modes!
// ============================================================================
class M3NavRailButton : public QPushButton {
    M3Icon icon_;
    QString label_;
    bool hovered_ = false;
    bool collapsed_ = false;
    qreal collapse_t_ = 0.0;

public:
    M3NavRailButton(M3Icon icon, const QString& label, QWidget* parent = nullptr)
        : QPushButton(parent), icon_(icon), label_(label) {
        setCheckable(true);
        setFixedHeight(38);
        setMinimumWidth(172);
        setCursor(Qt::PointingHandCursor);
        setToolTip(label);
    }

    void setCollapseProgress(qreal t) {
        const qreal clamped = std::clamp(t, 0.0, 1.0);
        if (std::abs(collapse_t_ - clamped) < 0.0005 && collapsed_ == (clamped >= 0.99)) return;
        collapse_t_ = clamped;
        collapsed_ = (clamped >= 0.99);
        const int h = static_cast<int>(std::round(38.0 + 4.0 * collapse_t_));
        const int min_w = static_cast<int>(std::round(172.0 + (42.0 - 172.0) * collapse_t_));
        setFixedHeight(h);
        setMinimumWidth(min_w);
        if (collapse_t_ < 0.01) {
            setMaximumWidth(16777215);
        } else {
            setMaximumWidth(min_w);
        }
        update();
    }

    void setCollapsed(bool c) {
        setCollapseProgress(c ? 1.0 : 0.0);
    }

protected:
    void enterEvent(QEnterEvent* e) override {
        hovered_ = true;
        update();
        QPushButton::enterEvent(e);
    }
    void leaveEvent(QEvent* e) override {
        hovered_ = false;
        update();
        QPushButton::leaveEvent(e);
    }
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.setRenderHint(QPainter::Antialiasing, true);

        const bool sel = isChecked();
        QRectF pillRect(0, 1, width(), height() - 2);
        const qreal rad = 18.0 + 2.0 * collapse_t_;
        if (sel) {
            p.setPen(Qt::NoPen);
            p.setBrush(QColor(255, 255, 255, 56));
            p.drawRoundedRect(pillRect, rad, rad);
        } else if (hovered_) {
            p.setPen(Qt::NoPen);
            p.setBrush(QColor(255, 255, 255, 30));
            p.drawRoundedRect(pillRect, rad, rad);
        }

        QColor fg = sel ? QColor(255, 255, 255, 255) : QColor(255, 255, 255, 232);
        const qreal ic_sz = 18.0 + 2.0 * collapse_t_;
        const qreal centered_x = (width() - ic_sz) / 2.0;
        const qreal ic_x = 14.0 + (centered_x - 14.0) * collapse_t_;
        const qreal ic_y = (height() - ic_sz) / 2.0;
        paint_md3_icon(&p, icon_, QRectF(ic_x, ic_y, ic_sz, ic_sz), fg);

        const qreal text_alpha = std::clamp(1.0 - collapse_t_ * 1.6, 0.0, 1.0);
        if (text_alpha <= 0.01) {
            return;
        }

        QFont f = font();
        f.setPixelSize(13);
        f.setWeight(sel ? QFont::Bold : QFont::Medium);
        p.setFont(f);
        QColor text_fg = fg;
        text_fg.setAlphaF(fg.alphaF() * text_alpha);
        p.setPen(text_fg);
        const qreal text_x = 42.0 - 8.0 * collapse_t_;
        const qreal avail_w = std::max(0.0, width() - text_x - 6.0);
        if (avail_w > 4.0) {
            p.save();
            p.setClipRect(QRectF(text_x, 0, avail_w, height()));
            p.drawText(QRectF(text_x, 0, 160.0, height()), Qt::AlignVCenter | Qt::AlignLeft, label_);
            p.restore();
        }
    }
};

// ============================================================================
// Cover Art Loader & Generative Warm Editorial Cover Cache
// ============================================================================
inline QHash<QString, QPixmap>& cover_cache() {
    static QHash<QString, QPixmap> cache;
    return cache;
}

inline QPixmap make_rounded_pixmap(const QPixmap& src, int size, int radius) {
    QPixmap out(size, size);
    out.fill(Qt::transparent);
    QPainter p(&out);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.setRenderHint(QPainter::SmoothPixmapTransform, true);
    QPainterPath path;
    path.addRoundedRect(QRectF(0, 0, size, size), radius, radius);
    p.setClipPath(path);
    p.drawPixmap(0, 0, size, size, src.scaled(size, size, Qt::KeepAspectRatioByExpanding, Qt::SmoothTransformation));
    return out;
}

inline QPixmap make_generative_cover(const QString& title, const QString& kind, int size, int radius) {
    // Editorial pastel palettes matching the reference screenshot's "New Albums" covers
    static const QPair<const char*, const char*> palettes[] = {
        {"#EAE8DF", "#212027"}, // Infinity style (warm cream + dark circle)
        {"#EBC8C4", "#D8A7A0"}, // The Glass style (blush pink architectural blocks)
        {"#D7E0E4", "#53748A"}, // Nevada style (cool mist + slate)
        {"#E4DDD7", "#B87A64"}, // Warm clay + terracotta
        {"#DCE3DC", "#5E7B68"}  // Soft sage + forest
    };
    uint h = qHash(title.isEmpty() ? QStringLiteral("Mineral") : title);
    const auto& pal = palettes[h % 5];
    QColor bg(pal.first);
    QColor fg(pal.second);

    QPixmap out(size, size);
    out.fill(Qt::transparent);
    QPainter p(&out);
    p.setRenderHint(QPainter::Antialiasing, true);
    QPainterPath clip;
    clip.addRoundedRect(QRectF(0, 0, size, size), radius, radius);
    p.setClipPath(clip);

    p.fillRect(0, 0, size, size, bg);

    // Minimalist geometric art inside cover (like "Infinity" & "The Glass")
    p.setPen(Qt::NoPen);
    if ((h % 3) == 0) {
        p.setBrush(fg);
        p.drawEllipse(QPointF(size * 0.5, size * 0.45), size * 0.28, size * 0.28);
    } else if ((h % 3) == 1) {
        p.setBrush(fg.lighter(115));
        p.drawRect(QRectF(size * 0.12, size * 0.38, size * 0.48, size * 0.62));
        p.setBrush(fg);
        p.drawRect(QRectF(size * 0.45, size * 0.55, size * 0.45, size * 0.45));
    } else {
        QLinearGradient g(0, 0, size, size);
        g.setColorAt(0.0, bg);
        g.setColorAt(1.0, fg.lighter(130));
        p.fillRect(0, 0, size, size, g);
        p.setBrush(QColor(255, 255, 255, 90));
        p.drawEllipse(QPointF(size * 0.68, size * 0.32), size * 0.24, size * 0.24);
    }

    p.setPen( (h % 3 == 0) ? QColor("#FFFFFF") : QColor("#2B2625") );
    QFont f = p.font();
    f.setWeight(QFont::Bold);
    f.setPixelSize(std::max(12, size / 4));
    p.setFont(f);
    QString glyph = QStringLiteral("♫");
    if (!title.trimmed().isEmpty()) {
        glyph = title.trimmed().left(1).toUpper();
    } else if (kind == "playlist") {
        glyph = QStringLiteral("▤");
    }
    p.drawText(QRect(0, 0, size, size), Qt::AlignCenter, glyph);
    return out;
}

inline QImage safe_read_image(const QString& path, int target_size) {
    QFileInfo fi(path);
    if (!fi.exists() || fi.size() < 64 || fi.size() > 12 * 1024 * 1024) return QImage();
    QImageReader reader(path);
    const QSize sz = reader.size();
    if (!sz.isValid() || sz.width() <= 0 || sz.height() <= 0 || sz.width() > 4096 || sz.height() > 4096) {
        return QImage();
    }
    if (sz.width() > target_size * 2 || sz.height() > target_size * 2) {
        reader.setScaledSize(sz.scaled(target_size * 2, target_size * 2, Qt::KeepAspectRatioByExpanding));
    }
    return reader.read();
}

inline QPixmap load_cover_pixmap(const QString& cover_path, const QString& title, const QString& kind, int size, int radius, QColor* out_seed = nullptr) {
    QFileInfo fi(cover_path);
    const bool file_ready = !cover_path.isEmpty() && fi.exists() && fi.size() >= 64;
    const QString cache_key = QString("%1|%2|%3|%4|%5").arg(cover_path).arg(file_ready ? 1 : 0).arg(title).arg(size).arg(radius);
    if (out_seed == nullptr && cover_cache().contains(cache_key)) {
        return cover_cache().value(cache_key);
    }
    if (!cover_path.isEmpty()) {
        QImage img = safe_read_image(cover_path, size);
        if (!img.isNull()) {
            if (out_seed) *out_seed = extract_seed_from_image(img);
            QPixmap pm = make_rounded_pixmap(QPixmap::fromImage(img), size, radius);
            if (cover_cache().size() > 400) cover_cache().clear();
            cover_cache().insert(cache_key, pm);
            return pm;
        }
    }
    if (out_seed) *out_seed = QColor();
    QPixmap fallback = make_generative_cover(title, kind, size, radius);
    if (cover_cache().size() > 400) cover_cache().clear();
    cover_cache().insert(cache_key, fallback);
    return fallback;
}

// ============================================================================
// Dynamic Color Block Hero Banner (`HeroBannerWidget`) + `CoverArtWidget`
// ============================================================================
inline QColor lerp_color(const QColor& a, const QColor& b, float t) {
    t = std::clamp(t, 0.0f, 1.0f);
    return QColor(
        static_cast<int>(a.red()   + (b.red()   - a.red())   * t),
        static_cast<int>(a.green() + (b.green() - a.green()) * t),
        static_cast<int>(a.blue()  + (b.blue()  - a.blue())  * t),
        static_cast<int>(a.alpha() + (b.alpha() - a.alpha()) * t)
    );
}

class HeroBannerWidget : public QFrame {
public:
    QColor curC1{"#DFCEC4"};
    QColor curC2{"#EFE6E0"};
    QColor curC3{"#E29578"};
    QColor targetC1{"#DFCEC4"};
    QColor targetC2{"#EFE6E0"};
    QColor targetC3{"#E29578"};
    float phase = 0.0f;
    float audio_energy = 0.0f;
    bool immersive_mode = false;
    int lerp_steps_ = 0;
    int tick_counter_ = 0;

    explicit HeroBannerWidget(QWidget* parent = nullptr) : QFrame(parent) {}

    void setTargetPalette(const QColor& c1, const QColor& c2, const QColor& c3) {
        if (targetC1 == c1 && targetC2 == c2 && targetC3 == c3) return;
        targetC1 = c1;
        targetC2 = c2;
        targetC3 = c3;
        lerp_steps_ = 24;
        update();
    }

    void tickDynamic(bool playing, float energy) {
        if (!playing && lerp_steps_ <= 0) return;
        audio_energy = audio_energy * 0.7f + std::clamp(energy, 0.0f, 1.0f) * 0.3f;
        if (playing) {
            phase += 0.055f + audio_energy * 0.045f;
            if (phase > 628.318f) phase -= 628.318f;
        }
        if (lerp_steps_ > 0) {
            --lerp_steps_;
            curC1 = lerp_color(curC1, targetC1, 0.22f);
            curC2 = lerp_color(curC2, targetC2, 0.22f);
            curC3 = lerp_color(curC3, targetC3, 0.22f);
        }
        update();
    }

protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.setRenderHint(QPainter::Antialiasing, true);
        const int w = width();
        const int h = height();
        if (w <= 4 || h <= 4) return;

        // 1. Base gradient across the Hero / Fullscreen canvas
        QLinearGradient baseGrad(0, 0, w, h);
        baseGrad.setColorAt(0.0, curC1);
        baseGrad.setColorAt(0.65, lerp_color(curC1, curC2, 0.6f));
        baseGrad.setColorAt(1.0, curC2);
        p.fillRect(rect(), baseGrad);

        // 2. Dynamic drifting color blocks (organic rounded color fields)
        p.setPen(Qt::NoPen);
        const float pulse = 1.0f + audio_energy * 0.22f;

        // Block A: Primary dynamic color block drifting across left-center
        {
            const float cx = w * (0.24f + 0.12f * std::sin(phase * 0.7f));
            const float cy = h * (0.45f + 0.18f * std::cos(phase * 0.5f));
            const float rx = std::max(w, h) * 0.44f * pulse;
            QRadialGradient rg(QPointF(cx, cy), rx);
            QColor cA = curC3;
            cA.setAlpha(immersive_mode ? 160 : 115);
            rg.setColorAt(0.0, cA);
            cA.setAlpha(0);
            rg.setColorAt(1.0, cA);
            p.setBrush(rg);
            p.drawRect(rect());
        }

        // Block B: Secondary tonal block drifting on right side behind artwork
        {
            const float cx = w * (0.76f + 0.11f * std::cos(phase * 0.6f + 1.2f));
            const float cy = h * (0.38f + 0.20f * std::sin(phase * 0.8f + 0.5f));
            const float rx = std::max(w, h) * 0.46f * pulse;
            QRadialGradient rg(QPointF(cx, cy), rx);
            QColor cB = lerp_color(curC1, curC3, 0.55f).darker(immersive_mode ? 118 : 108);
            cB.setAlpha(immersive_mode ? 165 : 120);
            rg.setColorAt(0.0, cB);
            cB.setAlpha(0);
            rg.setColorAt(1.0, cB);
            p.setBrush(rg);
            p.drawRect(rect());
        }
    }
};

// ============================================================================
// High-Performance Zero-QSS Hardware-Accelerated Volume Bar (`VolumeBarWidget`)
// ============================================================================
class VolumeBarWidget : public QWidget {
    int val_ = 100;
    int pre_mute_val_ = 80;
    bool dragging_ = false;
    bool hovered_ = false;
    QElapsedTimer throttle_timer_;
    QElapsedTimer interact_timer_;

    int pctFromX(qreal x) const {
        const qreal gx = 22.0;
        const qreal gw = std::max<qreal>(20.0, width() - gx - 6.0);
        const qreal ratio = std::clamp((x - gx) / gw, 0.0, 1.0);
        return static_cast<int>(std::round(ratio * 100.0));
    }

    void applyUserValue(int next_pct, bool force_emit) {
        next_pct = std::clamp(next_pct, 0, 100);
        if (next_pct > 0) pre_mute_val_ = next_pct;
        const bool changed = (val_ != next_pct);
        val_ = next_pct;
        interact_timer_.restart();
        if (changed) update();
        if (on_volume_change) {
            if (force_emit || !throttle_timer_.isValid() || throttle_timer_.elapsed() >= 50) {
                throttle_timer_.restart();
                on_volume_change(val_);
            }
        }
    }

public:
    std::function<void(int)> on_volume_change;

    explicit VolumeBarWidget(QWidget* parent = nullptr) : QWidget(parent) {
        setFixedSize(96, 24);
        setSizePolicy(QSizePolicy::Fixed, QSizePolicy::Fixed);
        setCursor(Qt::PointingHandCursor);
        setMouseTracking(true);
        setAttribute(Qt::WA_OpaquePaintEvent, false);
        setToolTip("Volume (Click, drag, scroll wheel, or click speaker to mute)");
    }

    QSize sizeHint() const override { return minimumSize(); }
    QSize minimumSizeHint() const override { return minimumSize(); }

    int value() const { return val_; }

    void setVolumeFromSnapshot(int pct) {
        if (dragging_) return;
        if (interact_timer_.isValid() && interact_timer_.elapsed() < 320) return;
        pct = std::clamp(pct, 0, 100);
        if (pct > 0) pre_mute_val_ = pct;
        if (val_ != pct) {
            val_ = pct;
            update();
        }
    }

protected:
    void enterEvent(QEnterEvent*) override {
        hovered_ = true;
        update();
    }

    void leaveEvent(QEvent*) override {
        hovered_ = false;
        update();
    }

    void mousePressEvent(QMouseEvent* ev) override {
        if (ev->button() != Qt::LeftButton) return;
        if (ev->position().x() < 20.0) {
            // Click speaker icon to toggle mute/unmute immediately
            const int target = (val_ > 0) ? 0 : std::max(10, pre_mute_val_);
            applyUserValue(target, true);
            return;
        }
        dragging_ = true;
        applyUserValue(pctFromX(ev->position().x()), false);
    }

    void mouseMoveEvent(QMouseEvent* ev) override {
        if (!dragging_) return;
        applyUserValue(pctFromX(ev->position().x()), false);
    }

    void mouseReleaseEvent(QMouseEvent* ev) override {
        if (ev->button() == Qt::LeftButton && dragging_) {
            dragging_ = false;
            applyUserValue(pctFromX(ev->position().x()), true);
        }
    }

    void wheelEvent(QWheelEvent* ev) override {
        const int delta = ev->angleDelta().y();
        if (delta == 0) return;
        const int step = (delta > 0) ? 5 : -5;
        applyUserValue(val_ + step, false);
        ev->accept();
    }

    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.setRenderHint(QPainter::Antialiasing, true);

        // 1. Speaker icon on the left (dims when muted)
        const QColor ic_col = (val_ == 0) ? QColor(255, 255, 255, 110) : QColor(255, 255, 255, 225);
        paint_md3_icon(&p, M3Icon::Volume, QRectF(1, 5, 14, 14), ic_col);
        if (val_ == 0) {
            p.setPen(QPen(m3.primaryDark, 1.8, Qt::SolidLine, Qt::RoundCap));
            p.drawLine(QPointF(2, 6), QPointF(14, 18));
        }

        // 2. Smooth hardware-accelerated groove & fill
        const qreal gx = 22.0;
        const qreal gw = std::max<qreal>(20.0, width() - gx - 6.0);
        const qreal gh = (hovered_ || dragging_) ? 5.0 : 3.5;
        const qreal gy = (height() - gh) * 0.5;

        p.setPen(Qt::NoPen);
        p.setBrush(QColor(255, 255, 255, 85));
        p.drawRoundedRect(QRectF(gx, gy, gw, gh), gh * 0.5, gh * 0.5);

        const qreal fill_w = gw * (std::clamp(val_, 0, 100) / 100.0);
        if (fill_w > 0.5) {
            p.setBrush(m3.primary);
            p.drawRoundedRect(QRectF(gx, gy, fill_w, gh), gh * 0.5, gh * 0.5);
        }

        // 3. Thumb handle
        const qreal tx = gx + fill_w;
        const qreal ty = height() * 0.5;
        const qreal tr = (hovered_ || dragging_) ? 5.5 : 4.5;
        p.setPen(QPen(m3.primaryDark, 1.5));
        p.setBrush(QColor("#FFFFFF"));
        p.drawEllipse(QPointF(tx, ty), tr, tr);
    }
};

class CoverArtWidget : public QWidget {
    QPixmap pixmap_;
    QString last_sig_;
    HeroBannerWidget* banner_ = nullptr;
    QColor last_prim_{"#EA957A"};
    QColor last_sec_{"#DFCEC4"};
    QColor last_tert_{"#E07A5F"};

public:
    explicit CoverArtWidget(QWidget* parent = nullptr) : QWidget(parent) {
        setMinimumSize(140, 140);
        setMaximumSize(360, 360);
    }

    void setBanner(HeroBannerWidget* b) { banner_ = b; }

    void applyDynamicPalette() {
        if (dynamic_cover_color) {
            m3 = M3ColorScheme::fromSeed(last_prim_, last_sec_, last_tert_);
            if (banner_) {
                banner_->setTargetPalette(m3.heroBgLeft, m3.heroBgRight, m3.tertiary);
            }
            if (on_m3_theme_changed) {
                on_m3_theme_changed();
            }
        } else if (banner_) {
            banner_->setTargetPalette(m3.heroBgLeft, m3.heroBgRight, m3.primary);
        }
    }

    void setCover(const QString& cover_path, const QString& title, const QString& kind) {
        QFileInfo fi(cover_path);
        const bool file_ready = !cover_path.isEmpty() && fi.exists() && fi.size() >= 64;
        const QString sig = cover_path + (file_ready ? "|1|" : "|0|") + title + "|" + kind;
        if (sig == last_sig_ && !pixmap_.isNull()) return;
        last_sig_ = sig;
        if (file_ready) {
            QImage img = safe_read_image(cover_path, 340);
            if (!img.isNull()) {
                pixmap_ = QPixmap::fromImage(img.scaled(340, 340, Qt::KeepAspectRatioByExpanding, Qt::SmoothTransformation));
                extract_palette_from_image(img, &last_prim_, &last_sec_, &last_tert_);
                applyDynamicPalette();
                update();
                return;
            }
        }
        pixmap_ = make_generative_cover(title, kind, 320, 14);
        extract_palette_from_image(pixmap_.toImage(), &last_prim_, &last_sec_, &last_tert_);
        applyDynamicPalette();
        update();
    }

protected:
    void paintEvent(QPaintEvent*) override {
        QPainter p(this);
        p.setRenderHint(QPainter::Antialiasing, true);
        p.setRenderHint(QPainter::SmoothPixmapTransform, true);
        if (pixmap_.isNull()) return;

        const int side = std::min(width(), height()) - 12;
        if (side <= 20) return;
        const int x = (width() - side) / 2;
        const int y = (height() - side) / 2;

        // Soft editorial shadow
        p.setPen(Qt::NoPen);
        p.setBrush(QColor(60, 40, 32, 38));
        p.drawRoundedRect(QRectF(x + 4, y + 8, side, side), 12, 12);

        QPainterPath clip;
        clip.addRoundedRect(QRectF(x, y, side, side), 12, 12);
        p.setClipPath(clip);
        p.drawPixmap(QRect(x, y, side, side), pixmap_);
    }
};

// ============================================================================
// Browse Table & New Albums 2x2 Grid Delegate (`MineralRowDelegate`)
// ============================================================================
class MineralRowDelegate : public QStyledItemDelegate {
public:
    explicit MineralRowDelegate(QObject* parent = nullptr) : QStyledItemDelegate(parent) {}

    void paint(QPainter* p, const QStyleOptionViewItem& opt, const QModelIndex& idx) const override {
        p->save();
        p->setRenderHint(QPainter::Antialiasing, true);

        const bool grid_mode = idx.data(RoleGridMode).toBool();
        const bool selected = (opt.state & QStyle::State_Selected);
        const bool hovered = (opt.state & QStyle::State_MouseOver);
        const bool current = idx.data(RoleCurrent).toBool();
        const bool loved = idx.data(RoleLoved).toBool();

        const QString title = idx.data(RoleTitle).toString();
        const QString subtitle = idx.data(RoleSubtitle).toString();
        const QString album = idx.data(RoleAlbum).toString();
        const QString source = idx.data(RoleSource).toString();
        const QString duration = idx.data(RoleDuration).toString();
        const QString kind = idx.data(RoleKind).toString();
        const QString cover = idx.data(RoleCover).toString();

        if (grid_mode) {
            // "New Albums" 2x2 Large Square Cover Grid:
            // Shows ONLY the 4 large covers by default; reveals Title & Artist overlay ONLY on mouse hover!
            const int cov_size = std::min(opt.rect.width(), opt.rect.height()) - 10;
            if (cov_size <= 20) {
                p->restore();
                return;
            }
            const int cov_x = opt.rect.left() + (opt.rect.width() - cov_size) / 2;
            const int cov_y = opt.rect.top() + (opt.rect.height() - cov_size) / 2;
            const QRectF cov_rect(cov_x, cov_y, cov_size, cov_size);

            // Soft cover shadow
            p->setPen(Qt::NoPen);
            p->setBrush(QColor(43, 38, 37, hovered ? 32 : 16));
            p->drawRoundedRect(cov_rect.adjusted(1, 3, 1, 4), 8, 8);

            QPixmap pm = load_cover_pixmap(cover, title, kind, cov_size, 8);
            p->drawPixmap(cov_x, cov_y, cov_size, cov_size, pm);

            // Hover-only Title & Subtitle overlay on top of the cover art
            if (hovered) {
                QPainterPath clip;
                clip.addRoundedRect(cov_rect, 8, 8);
                p->setClipPath(clip);

                QLinearGradient overlayGrad(cov_x, cov_y + cov_size * 0.35, cov_x, cov_y + cov_size);
                overlayGrad.setColorAt(0.0, QColor(28, 22, 20, 0));
                overlayGrad.setColorAt(0.45, QColor(28, 22, 20, 155));
                overlayGrad.setColorAt(1.0, QColor(28, 22, 20, 225));
                p->setPen(Qt::NoPen);
                p->setBrush(overlayGrad);
                p->drawRect(cov_rect);

                QFont tf = opt.font;
                tf.setPixelSize(13);
                tf.setWeight(QFont::Bold);
                p->setFont(tf);
                p->setPen(QColor("#FFFFFF"));
                const int pad = 10;
                const int text_w = cov_size - pad * 2;
                QRect tr(cov_x + pad, cov_y + cov_size - (subtitle.isEmpty() ? 28 : 44), text_w, 18);
                p->drawText(tr, Qt::AlignLeft | Qt::AlignVCenter,
                            QFontMetrics(tf).elidedText(title, Qt::ElideRight, text_w));

                if (!subtitle.isEmpty()) {
                    QFont sf = opt.font;
                    sf.setPixelSize(11);
                    sf.setWeight(QFont::Medium);
                    p->setFont(sf);
                    p->setPen(QColor(255, 255, 255, 210));
                    QRect sr(cov_x + pad, cov_y + cov_size - 24, text_w, 16);
                    p->drawText(sr, Qt::AlignLeft | Qt::AlignVCenter,
                                QFontMetrics(sf).elidedText(subtitle, Qt::ElideRight, text_w));
                }
            } else if (selected || current) {
                // Subtle coral ring when selected/playing without text clutter
                p->setPen(QPen(m3.primaryDark, 2.0));
                p->setBrush(Qt::NoBrush);
                p->drawRoundedRect(cov_rect.adjusted(1, 1, -1, -1), 8, 8);
            }

            p->restore();
            return;
        }

        QRectF r = QRectF(opt.rect).adjusted(6, 3, -6, -3);

        // "Browse" Table Row (matching "Your Time - Nick Murphy" floating white card on selection!)
        if (selected || current) {
            // Multi-layer diffused drop shadow for floating white card
            p->setPen(Qt::NoPen);
            p->setBrush(QColor(43, 38, 37, 8));
            p->drawRoundedRect(r.adjusted(-2, 0, 2, 4), 10, 10);
            p->setBrush(QColor(43, 38, 37, 14));
            p->drawRoundedRect(r.adjusted(0, 1, 0, 2), 8, 8);

            p->setPen(QPen(QColor("#F0EAE6"), 1.0));
            p->setBrush(QColor("#FFFFFF"));
            p->drawRoundedRect(r, 8, 8);
        } else if (hovered) {
            p->setPen(Qt::NoPen);
            p->setBrush(QColor("#FAF7F5"));
            p->drawRoundedRect(r, 8, 8);
        }

        const int y_mid = static_cast<int>(r.center().y());
        int x = static_cast<int>(r.left()) + 12;

        // 1. 40x40px Square/4px-Rounded Album Cover Thumbnail (exact match to screenshot)
        QPixmap thumb = load_cover_pixmap(cover, title, kind, 40, 4);
        p->drawPixmap(x, y_mid - 20, 40, 40, thumb);
        if (current) {
            // Coral playing indicator dot/icon on cover
            p->setPen(Qt::NoPen);
            p->setBrush(QColor(234, 149, 122, 215));
            p->drawRoundedRect(QRectF(x, y_mid - 20, 40, 40), 4, 4);
            paint_md3_icon(p, M3Icon::Equalizer, QRectF(x + 11, y_mid - 9, 18, 18), QColor("#FFFFFF"));
        } else if (hovered && kind == "setting") {
            p->setPen(Qt::NoPen);
            p->setBrush(QColor(43, 38, 37, 130));
            p->drawRoundedRect(QRectF(x, y_mid - 20, 40, 40), 4, 4);
            paint_md3_icon(p, M3Icon::Settings, QRectF(x + 12, y_mid - 8, 16, 16), QColor("#FFFFFF"));
        } else if (hovered && kind != "source") {
            // Hover play overlay on thumbnail for instant single-click playback
            p->setPen(Qt::NoPen);
            p->setBrush(QColor(43, 38, 37, 130));
            p->drawRoundedRect(QRectF(x, y_mid - 20, 40, 40), 4, 4);
            paint_md3_icon(p, M3Icon::Play, QRectF(x + 12, y_mid - 8, 16, 16), QColor("#FFFFFF"));
        }
        x += 56;

        const bool show_quick_actions = hovered && (kind == "track" || kind == "song" || kind.isEmpty());
        const bool is_setting_row = (kind == "setting");
        const int total_w = static_cast<int>(r.right()) - x - 14;
        const int len_w = show_quick_actions ? 104 : (is_setting_row ? 72 : (duration.isEmpty() ? 0 : 48));
        const int rem_w = std::max(100, total_w - len_w - 12);

        const int title_w = album.isEmpty() ? static_cast<int>(rem_w * 0.56) : static_cast<int>(rem_w * 0.42);
        const int artist_w = album.isEmpty() ? (rem_w - title_w) : static_cast<int>(rem_w * 0.30);
        const int album_w = album.isEmpty() ? 0 : (rem_w - title_w - artist_w);

        // 2. Title (13px Bold #2B2625)
        QFont tf = opt.font;
        tf.setPixelSize(13);
        tf.setWeight(QFont::DemiBold);
        p->setFont(tf);
        p->setPen(current ? m3.primaryDark : m3.onSurface);
        QString disp_title = loved ? (title + "  ♥") : title;
        QRect title_rect(x, static_cast<int>(r.top()), title_w - 12, static_cast<int>(r.height()));
        p->drawText(title_rect, Qt::AlignVCenter | Qt::AlignLeft,
                    QFontMetrics(tf).elidedText(disp_title, Qt::ElideRight, title_rect.width()));
        x += title_w;

        // 3. Artist (13px Medium #4A4340)
        QFont sf = opt.font;
        sf.setPixelSize(13);
        sf.setWeight(QFont::Medium);
        p->setFont(sf);
        p->setPen(QColor("#4A4340"));
        QRect sub_rect(x, static_cast<int>(r.top()), artist_w - 12, static_cast<int>(r.height()));
        p->drawText(sub_rect, Qt::AlignVCenter | Qt::AlignLeft,
                    QFontMetrics(sf).elidedText(subtitle, Qt::ElideRight, sub_rect.width()));
        x += artist_w;

        // 4. Album (13px Regular #6E6662)
        if (album_w > 0) {
            sf.setWeight(QFont::Normal);
            p->setFont(sf);
            p->setPen(m3.onSurfaceVariant);
            QString alb_text = album.isEmpty() ? source : album;
            QRect alb_rect(x, static_cast<int>(r.top()), album_w - 12, static_cast<int>(r.height()));
            p->drawText(alb_rect, Qt::AlignVCenter | Qt::AlignLeft,
                        QFontMetrics(sf).elidedText(alb_text, Qt::ElideRight, alb_rect.width()));
        }

        // 5. Hover Quick-Action Buttons (Play / Favourite / Add to Queue) or Duration
        if (show_quick_actions) {
            const int btn_s = 26;
            const int gap = 6;
            int bx = static_cast<int>(r.right()) - 12 - (btn_s * 3 + gap * 2);
            const int by = y_mid - btn_s / 2;

            // Button 1: Play Now
            QRectF b_play(bx, by, btn_s, btn_s);
            p->setPen(Qt::NoPen);
            p->setBrush(m3.primary);
            p->drawRoundedRect(b_play, 13, 13);
            paint_md3_icon(p, M3Icon::Play, b_play.adjusted(6, 6, -6, -6), QColor("#FFFFFF"));
            bx += btn_s + gap;

            // Button 2: Favourite
            QRectF b_fav(bx, by, btn_s, btn_s);
            p->setPen(QPen(QColor("#E8DFD9"), 1.0));
            p->setBrush(loved ? m3.secondaryContainer : QColor("#FFFFFF"));
            p->drawRoundedRect(b_fav, 13, 13);
            paint_md3_icon(p, loved ? M3Icon::HeartFilled : M3Icon::HeartOutline, b_fav.adjusted(6, 6, -6, -6), loved ? m3.primaryDark : m3.onSurfaceVariant);
            bx += btn_s + gap;

            // Button 3: Add to Queue (+)
            QRectF b_queue(bx, by, btn_s, btn_s);
            p->setPen(QPen(QColor("#E8DFD9"), 1.0));
            p->setBrush(QColor("#FFFFFF"));
            p->drawRoundedRect(b_queue, 13, 13);
            p->setPen(QPen(m3.onSurfaceVariant, 1.7, Qt::SolidLine, Qt::RoundCap));
            p->drawLine(QPointF(b_queue.center().x() - 4.5, b_queue.center().y()),
                        QPointF(b_queue.center().x() + 4.5, b_queue.center().y()));
            p->drawLine(QPointF(b_queue.center().x(), b_queue.center().y() - 4.5),
                        QPointF(b_queue.center().x(), b_queue.center().y() + 4.5));
        } else if (is_setting_row) {
            QString badge_text = QStringLiteral("Edit");
            if (source.startsWith("cycle|") || source == "ui") badge_text = QStringLiteral("Cycle");
            else if (source.startsWith("toggle|")) badge_text = QStringLiteral("Toggle");
            else if (source.startsWith("dir|") || source.startsWith("dirs|")) badge_text = QStringLiteral("Browse");
            else if (source.startsWith("lua|")) badge_text = QStringLiteral("Script");
            else if (source.startsWith("action|")) badge_text = QStringLiteral("Action");

            const int pill_w = 60;
            const int pill_h = 24;
            const QRectF pill_rect(r.right() - pill_w - 12, y_mid - pill_h / 2.0, pill_w, pill_h);
            p->setPen(QPen(hovered ? m3.primaryDark : QColor("#E4DCD6"), 1.0));
            p->setBrush(hovered ? m3.primary : QColor("#FAF7F5"));
            p->drawRoundedRect(pill_rect, 12, 12);

            QFont pf = opt.font;
            pf.setPixelSize(11);
            pf.setWeight(QFont::DemiBold);
            p->setFont(pf);
            p->setPen(hovered ? QColor("#FFFFFF") : m3.onSurfaceVariant);
            p->drawText(pill_rect, Qt::AlignCenter, badge_text);
        } else if (!duration.isEmpty()) {
            QFont df = opt.font;
            df.setPixelSize(12);
            p->setFont(df);
            p->setPen(m3.mutedText);
            QRect dur_rect(static_cast<int>(r.right()) - len_w - 10, static_cast<int>(r.top()), len_w, static_cast<int>(r.height()));
            p->drawText(dur_rect, Qt::AlignVCenter | Qt::AlignRight, duration);
        }

        p->restore();
    }
};

// ============================================================================
// Warm Editorial Dual-Line Synced Lyrics Delegate (`LyricDelegate`)
// Supports Adjustable Fullscreen Font Size & Silky Active-Line Transitions
// ============================================================================
inline int fullscreen_lyric_font_px = 20;
inline int normal_lyric_font_px = 15;

class LyricDelegate : public QStyledItemDelegate {
    QListWidget* list_;

public:
    explicit LyricDelegate(QListWidget* list) : QStyledItemDelegate(list), list_(list) {}

    QSize sizeHint(const QStyleOptionViewItem&, const QModelIndex& idx) const override {
        const bool hero_mode = list_ && list_->property("hero_mode").toBool();
        const QString sub_text = idx.data(RoleSubtitle).toString();
        const int base_px = hero_mode ? fullscreen_lyric_font_px : normal_lyric_font_px;
        if (sub_text.isEmpty()) {
            return QSize(0, base_px + (hero_mode ? 26 : 22));
        }
        const int sub_px = std::max(11, base_px - (hero_mode ? 5 : 3));
        return QSize(0, base_px + sub_px + (hero_mode ? 32 : 26));
    }

    void paint(QPainter* p, const QStyleOptionViewItem& opt, const QModelIndex& idx) const override {
        p->save();
        p->setRenderHint(QPainter::Antialiasing, true);
        p->setRenderHint(QPainter::TextAntialiasing, true);

        const qreal subpixel_y = (list_ && list_->property("subpixel_y").isValid())
            ? list_->property("subpixel_y").toReal()
            : 0.0;
        p->translate(0.0, -subpixel_y);

        const QString text = idx.data(RoleTitle).toString();
        const QString sub_text = idx.data(RoleSubtitle).toString();
        const bool active = idx.data(RoleCurrent).toBool();
        const bool hero_mode = list_ && list_->property("hero_mode").toBool();
        const int active_row = list_ ? list_->property("active_row").toInt() : -1;
        const int row = idx.row();

        const double smooth_row = (list_ && list_->property("smooth_row").isValid())
            ? list_->property("smooth_row").toDouble()
            : double(active_row);
        const double dist = (active_row >= 0) ? std::abs(double(row) - smooth_row) : 2.0;
        const double raw_w = (active_row >= 0) ? std::clamp(1.0 - dist, 0.0, 1.0) : (active ? 1.0 : 0.0);
        const double w = raw_w * raw_w * (3.0 - 2.0 * raw_w);

        QRectF r = QRectF(opt.rect).adjusted(hero_mode ? 0 : 6, 3, -6, -3);

        if (w > 0.01) {
            p->setPen(Qt::NoPen);
            const int bg_alpha = static_cast<int>(std::round((hero_mode ? 44.0 : 255.0) * w));
            QColor bg = hero_mode ? QColor(255, 255, 255, bg_alpha) : m3.secondaryContainer;
            if (!hero_mode) bg.setAlpha(bg_alpha);
            p->setBrush(bg);
            p->drawRoundedRect(r, 10, 10);
        } else if (opt.state & QStyle::State_MouseOver) {
            p->setPen(Qt::NoPen);
            p->setBrush(hero_mode ? QColor(255, 255, 255, 22) : m3.surfaceContainerLow);
            p->drawRoundedRect(r, 10, 10);
        }

        double alpha_f = 255.0;
        if (active_row >= 0) {
            if (dist <= 1.0) alpha_f = 255.0 - 60.0 * dist;
            else if (dist <= 2.0) alpha_f = 195.0 - 55.0 * (dist - 1.0);
            else if (dist <= 3.0) alpha_f = 140.0 - 42.0 * (dist - 2.0);
            else alpha_f = std::max(68.0, 98.0 - 15.0 * (dist - 3.0));
        }
        const int alpha = std::clamp(static_cast<int>(std::round(alpha_f)), 55, 255);

        QColor main_color;
        if (hero_mode) {
            main_color = QColor("#FFFFFF");
        } else {
            main_color = QColor(
                static_cast<int>(std::round(m3.onSurface.red() + (m3.primaryDark.red() - m3.onSurface.red()) * w)),
                static_cast<int>(std::round(m3.onSurface.green() + (m3.primaryDark.green() - m3.onSurface.green()) * w)),
                static_cast<int>(std::round(m3.onSurface.blue() + (m3.primaryDark.blue() - m3.onSurface.blue()) * w))
            );
        }
        main_color.setAlpha(alpha);

        const int base_px = hero_mode ? fullscreen_lyric_font_px : normal_lyric_font_px;
        const int inactive_px = std::max(12, base_px - (hero_mode ? 3 : 2));
        const qreal scale = 1.0 + (qreal(base_px - inactive_px) / qreal(inactive_px)) * w;

        // Sub-pixel floating-point scale around left-center of the lyric line (zero integer font snapping!)
        const QPointF pivot(r.left() + 12.0, r.center().y());
        p->translate(pivot);
        p->scale(scale, scale);
        p->translate(-pivot);

        QFont mf = opt.font;
        mf.setPixelSize(inactive_px);
        mf.setWeight(w > 0.45 ? QFont::Bold : QFont::Medium);
        p->setFont(mf);
        p->setPen(main_color);

        const int avail_w = std::max(40, static_cast<int>((r.width() - 28.0) / scale));

        if (sub_text.isEmpty()) {
            QRectF text_r(r.left() + 12.0, r.top(), avail_w, r.height());
            p->drawText(text_r, Qt::AlignVCenter | Qt::AlignLeft,
                        QFontMetrics(mf).elidedText(text, Qt::ElideRight, avail_w));
        } else {
            const int main_h = inactive_px + 7;
            const int sub_px = std::max(11, inactive_px - (hero_mode ? 4 : 2));
            const int sub_h = sub_px + 6;
            const qreal total_text_h = main_h + sub_h + 2;
            const qreal start_y = r.top() + std::max<qreal>(2.0, (r.height() - total_text_h) * 0.5);

            QRectF top_r(r.left() + 12.0, start_y, avail_w, main_h);
            p->drawText(top_r, Qt::AlignVCenter | Qt::AlignLeft,
                        QFontMetrics(mf).elidedText(text, Qt::ElideRight, avail_w));

            QFont sf = opt.font;
            sf.setPixelSize(sub_px);
            p->setFont(sf);
            QColor sub_color = hero_mode
                ? QColor("#FAF6F4")
                : (w > 0.5 ? m3.onSecondaryContainer : m3.onSurfaceVariant);
            sub_color.setAlpha(std::max(55, alpha - 25));
            p->setPen(sub_color);
            QRectF bot_r(r.left() + 12.0, start_y + main_h + 2, avail_w, sub_h);
            p->drawText(bot_r, Qt::AlignVCenter | Qt::AlignLeft,
                        QFontMetrics(sf).elidedText(sub_text, Qt::ElideRight, avail_w));
        }

        p->restore();
    }
};

} // namespace mineral_ui
