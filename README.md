# voice2blog - Voice-Driven Technical Blogging Tool

[![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

Convert spoken technical content into well-structured Markdown documents using Whisper API and LLM.

## Features

- 🎙️ Real-time audio recording with automatic WAV file generation
- 🤖 AI-powered speech-to-text transcription (Whisper API)
- 📝 Automatic conversion of raw transcripts to structured Markdown
- 🔧 Configurable templates for different documentation styles
- 🚀 Efficient async API communication

## Installation

### Prerequisites
- Rust 1.65+
- [Whisper API Key](https://platform.openai.com/docs/guides/speech-to-text)
- [LLM API Key](https://groq.com/)

```bash
git clone https://github.com/yourusername/voice2blog.git
cd voice2blog
cp .env.example .env
# Edit .env with your API keys
```

## Configuration

### API Configuration
```json
// config/whisper.json
{
  "url": "https://api.openai.com/v1/audio/transcriptions",
  "model": "whisper-large-v3"
}

// config/llm.json
{
  "url": "https://api.groq.com/openai/v1/chat/completions",
  "model": "deepseek-r1-distill-llama-70b",
  "system_prompt": "...",
  "user_prompt_template": "..."
}
```

### Environment Variables (`.env`)
```
WHISPER_API_KEY=your_whisper_key
LLM_API_KEY=your_llm_key
RECORDING_FILENAME=recording.wav
OUTPUT_FILENAME=output.md
```

## Usage

```bash
cargo run

# Recording control:
# - Press Ctrl+C to stop recording
# - Automatic processing starts after stopping
```

Sample output structure:
```markdown
## Deep Learning Pipeline Optimization

Key components:
1. **Data Preprocessing**
   - Normalization: `(data - mean)/std`
   - Augmentation techniques:
     - Random cropping
     - Color jitter

2. Model Architecture
```python
class EfficientNet(nn.Module):
    def __init__(self):
        super().__init__()
        # MBConv blocks...
```

## Key Features

- Strict preservation of technical details
- Automatic code block detection
- Context-aware formatting
- Multi-level heading structure
- Smart list generation

## Notes

1. API costs may apply for Whisper and LLM usage
2. Default recording format: 16kHz mono WAV
3. Supported LLM providers: Groq, OpenAI, etc.
4. Typical processing time: 30-60s per minute of audio

## Contributing

PRs welcome! Please follow:
1. Rustfmt formatting rules
2. Comprehensive test coverage
3. Clear documentation
4. Semantic commit messages

## License

MIT License © 2024 Zhenyu Huang

```

This README provides essential information while maintaining technical professionalism. Adjustments can be made for specific API provider requirements or additional features.
