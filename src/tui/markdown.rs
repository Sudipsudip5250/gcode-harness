pub use gcode_tui_markdown::{
    CopyTargetKind, IncrementalMarkdownRenderer, MarkdownDebugStats, MarkdownMemoryProfile,
    RawCopyTarget, center_code_blocks, debug_memory_profile, debug_stats, debug_stats_json,
    extract_copy_targets_from_rendered_lines, highlight_file_lines, highlight_line, progress_bar,
    progress_line, recenter_structured_blocks_for_display, render_markdown, render_markdown_lazy,
    render_markdown_with_width, render_table_with_width, reset_debug_stats, set_center_code_blocks,
    wrap_line, wrap_lines,
};

fn to_markdown_diagram_mode(
    mode: crate::config::DiagramDisplayMode,
) -> gcode_tui_markdown::DiagramDisplayMode {
    match mode {
        crate::config::DiagramDisplayMode::None => gcode_tui_markdown::DiagramDisplayMode::None,
        crate::config::DiagramDisplayMode::Margin => gcode_tui_markdown::DiagramDisplayMode::Margin,
        crate::config::DiagramDisplayMode::Pinned => gcode_tui_markdown::DiagramDisplayMode::Pinned,
    }
}

fn from_markdown_diagram_mode(
    mode: gcode_tui_markdown::DiagramDisplayMode,
) -> crate::config::DiagramDisplayMode {
    match mode {
        gcode_tui_markdown::DiagramDisplayMode::None => crate::config::DiagramDisplayMode::None,
        gcode_tui_markdown::DiagramDisplayMode::Margin => crate::config::DiagramDisplayMode::Margin,
        gcode_tui_markdown::DiagramDisplayMode::Pinned => crate::config::DiagramDisplayMode::Pinned,
    }
}

fn to_markdown_spacing_mode(
    mode: crate::config::MarkdownSpacingMode,
) -> gcode_tui_markdown::MarkdownSpacingMode {
    match mode {
        crate::config::MarkdownSpacingMode::Compact => {
            gcode_tui_markdown::MarkdownSpacingMode::Compact
        }
        crate::config::MarkdownSpacingMode::Document => {
            gcode_tui_markdown::MarkdownSpacingMode::Document
        }
    }
}

pub fn install_gcode_markdown_hooks() {
    gcode_tui_markdown::set_config_snapshot_hook(|| {
        let cfg = crate::config::config();
        gcode_tui_markdown::MarkdownConfigSnapshot {
            diagram_mode: to_markdown_diagram_mode(cfg.display.diagram_mode),
            markdown_spacing: to_markdown_spacing_mode(cfg.display.markdown_spacing),
        }
    });
    gcode_tui_markdown::set_memory_snapshot_hook(|| {
        let snapshot = crate::process_memory::snapshot_with_source("client:markdown:memory");
        gcode_tui_markdown::ProcessMemorySnapshot {
            rss_bytes: snapshot.rss_bytes,
            peak_rss_bytes: snapshot.peak_rss_bytes,
            virtual_bytes: snapshot.virtual_bytes,
        }
    });
}

pub fn set_diagram_mode_override(mode: Option<crate::config::DiagramDisplayMode>) {
    gcode_tui_markdown::set_diagram_mode_override(mode.map(to_markdown_diagram_mode));
}

pub fn get_diagram_mode_override() -> Option<crate::config::DiagramDisplayMode> {
    gcode_tui_markdown::get_diagram_mode_override().map(from_markdown_diagram_mode)
}

pub fn with_deferred_mermaid_render_context<T>(f: impl FnOnce() -> T) -> T {
    gcode_tui_markdown::with_deferred_mermaid_render_context(f)
}
