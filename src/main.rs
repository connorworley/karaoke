use winrt::*;
include_bindings!();

async fn create_audio_graph() -> std::result::Result<windows::media::audio::AudioGraph, winrt::Error>
{
    let settings = windows::media::audio::AudioGraphSettings::create(
        windows::media::render::AudioRenderCategory::Media,
    )?;
    settings.set_quantum_size_selection_mode(
        windows::media::audio::QuantumSizeSelectionMode::LowestLatency,
    )?;
    let graph = windows::media::audio::AudioGraph::create_async(settings)?
        .await?
        .graph()?;
    let input_node = graph
        .create_device_input_node_async(windows::media::capture::MediaCategory::Other)?
        .await?
        .device_input_node()?;
    let output_node = graph
        .create_device_output_node_async()?
        .await?
        .device_output_node()?;
    input_node.add_outgoing_connection(&output_node)?;

    let reverb = windows::media::audio::ReverbEffectDefinition::create(&graph)?;
    output_node.effect_definitions()?.append(reverb)?;
    
    Ok(graph)
}

fn main() -> std::result::Result<(), winrt::Error> {
    let graph = futures::executor::block_on(create_audio_graph())?;
    graph.start()?;
    std::thread::park();
    graph.stop()?;
    Ok(())
}
