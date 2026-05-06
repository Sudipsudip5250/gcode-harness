pub use gcode_tui_mermaid::*;

pub fn install_gcode_mermaid_hooks() {
    gcode_tui_mermaid::set_log_hooks(crate::logging::info, crate::logging::warn);
    gcode_tui_mermaid::set_render_completed_hook(|| {
        crate::bus::Bus::global().publish(crate::bus::BusEvent::MermaidRenderCompleted);
    });
    gcode_tui_mermaid::set_memory_snapshot_hook(|| {
        let snapshot = crate::process_memory::snapshot_with_source("client:mermaid:memory");
        gcode_tui_mermaid::ProcessMemorySnapshot {
            rss_bytes: snapshot.rss_bytes,
            peak_rss_bytes: snapshot.peak_rss_bytes,
            virtual_bytes: snapshot.virtual_bytes,
        }
    });
}
