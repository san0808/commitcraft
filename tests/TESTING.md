# Testing Guide for CommitCraft

This guide explains how to test CommitCraft's AI provider implementations and ensure they work correctly with real APIs.

## Quick Test Commands

```bash
# Run all unit tests (no API keys required)
cargo test --lib

# Run integration tests (requires API keys)
cargo test --test integration_tests --ignored

# Run specific provider integration test
cargo test --test integration_tests test_openai_integration --ignored
cargo test --test integration_tests test_gemini_integration --ignored  
cargo test --test integration_tests test_anthropic_integration --ignored

# Run all tests including integration (requires all API keys)
cargo test --test integration_tests test_all_providers_consistency --ignored
```

## Test Structure

### Unit Tests (`tests/unit_tests.rs`)
- **No API keys required** - Safe to run anytime
- Tests provider initialization
- Tests response parsing logic
- Tests JSON schema generation
- Tests conventional commit format validation
- Uses mock providers for basic functionality

### Integration Tests (`tests/integration_tests.rs`)
- **Requires real API keys** - Tests actual API communication
- Tests all three providers: OpenAI, Gemini, Anthropic
- Validates real API responses
- Ensures consistent behavior across providers
- Marked with `#[ignore]` to prevent accidental runs

## Setting Up API Keys for Integration Tests

To run integration tests, set these environment variables:

```bash
# OpenAI API Key
export OPENAI_API_KEY="sk-your-openai-key-here"

# Google AI Studio API Key (for Gemini)
export GOOGLE_AI_API_KEY="your-google-ai-key-here"

# Anthropic API Key
export ANTHROPIC_API_KEY="sk-ant-your-anthropic-key-here"
```

### Getting API Keys

1. **OpenAI**: Get from [OpenAI Platform](https://platform.openai.com/api-keys)
2. **Gemini**: Get from [Google AI Studio](https://makersuite.google.com/app/apikey)
3. **Anthropic**: Get from [Anthropic Console](https://console.anthropic.com/)

## Test Categories

### 🔧 Provider Implementation Tests

Each provider is tested for:
- ✅ **Function/Tool Calling**: Proper use of AI provider's tool calling APIs
- ✅ **Response Parsing**: Correct parsing of function call responses
- ✅ **Fallback Handling**: Graceful fallback to text parsing if function calling fails
- ✅ **Error Handling**: Proper error messages and recovery
- ✅ **Schema Validation**: JSON schema generation and validation

### 📋 Conventional Commit Validation

Tests ensure all providers return:
- ✅ **Proper Format**: `<type>[scope]: <description>` format
- ✅ **Length Limits**: Title ≤ 50 characters
- ✅ **Valid Types**: feat, fix, docs, style, refactor, test, chore
- ✅ **Non-empty Description**: Detailed explanation required

### 🔄 API Compatibility Tests

Integration tests verify:
- ✅ **Real API Communication**: Actual HTTP requests to provider APIs
- ✅ **Response Format**: Current API response structure compatibility
- ✅ **Model Availability**: Configured models are accessible
- ✅ **Rate Limiting**: Proper handling of API limits

## Running Tests

### 1. Run Unit Tests Only (Safe)
```bash
cargo test --lib
```
**Output Example:**
```
test unit_tests::test_provider_initialization ... ok
test unit_tests::test_openai_response_parsing ... ok
test unit_tests::test_gemini_response_parsing ... ok
test unit_tests::test_anthropic_response_parsing ... ok
test unit_tests::test_conventional_commit_format_validation ... ok
test unit_tests::test_json_schema_generation ... ok
test mock_tests::test_mock_provider_success ... ok
test mock_tests::test_mock_provider_failure ... ok
```

### 2. Run Single Provider Integration Test
```bash
# Test OpenAI only
OPENAI_API_KEY="sk-..." cargo test --test integration_tests test_openai_integration --ignored

# Test Gemini only  
GOOGLE_AI_API_KEY="..." cargo test --test integration_tests test_gemini_integration --ignored

# Test Anthropic only
ANTHROPIC_API_KEY="sk-ant-..." cargo test --test integration_tests test_anthropic_integration --ignored
```

### 3. Run All Provider Comparison Test
```bash
# Set all API keys and test consistency across providers
export OPENAI_API_KEY="sk-..."
export GOOGLE_AI_API_KEY="..."
export ANTHROPIC_API_KEY="sk-ant-..."

cargo test --test integration_tests test_all_providers_consistency --ignored
```

**Output Example:**
```
Testing OpenAI provider...
OpenAI - Title: feat: add new feature
OpenAI - Description: Add new functionality to process files with improved error handling

Testing Gemini provider...
Gemini - Title: feat: add file processing
Gemini - Description: Implement file processing functionality with proper error handling

Testing Anthropic provider...
Anthropic - Title: feat: implement file processing
Anthropic - Description: Add new file processing capability with error handling and logging
```

## Understanding Test Results

### ✅ Successful Integration Test
```
test test_openai_integration ... ok
```
- Provider API is working correctly
- Function calling is properly implemented
- Response parsing is successful
- Commit format validation passes

### ❌ Failed Integration Test
```
test test_openai_integration ... FAILED
```
Common failure reasons:
1. **Invalid API Key**: Check your environment variables
2. **API Format Change**: Provider changed their response format
3. **Network Issues**: Connection problems
4. **Rate Limiting**: Too many requests too quickly
5. **Model Unavailable**: Specified model is not accessible

### 🔍 Debugging Failed Tests

1. **Check API Keys**:
```bash
echo $OPENAI_API_KEY
echo $GOOGLE_AI_API_KEY  
echo $ANTHROPIC_API_KEY
```

2. **Test API Connectivity**:
```bash
# Test OpenAI
curl -H "Authorization: Bearer $OPENAI_API_KEY" https://api.openai.com/v1/models

# Test Gemini
curl "https://generativelanguage.googleapis.com/v1beta/models?key=$GOOGLE_AI_API_KEY"

# Test Anthropic
curl -H "x-api-key: $ANTHROPIC_API_KEY" -H "anthropic-version: 2024-06-01" https://api.anthropic.com/v1/messages
```

3. **Run with Debug Output**:
```bash
RUST_LOG=debug cargo test --test integration_tests test_openai_integration --ignored
```

## What Our Tests Verify

### 1. Updated API Implementations
- ✅ **OpenAI**: Uses latest function calling with `tools` parameter
- ✅ **Gemini**: Uses latest function declarations with `tool_config`
- ✅ **Anthropic**: Uses latest tool calling with `input_schema`

### 2. Robust Error Handling
- ✅ **Function Call Parsing**: Primary method for structured responses
- ✅ **Text Fallback**: Secondary parsing for JSON in text responses
- ✅ **Detailed Error Messages**: Clear failure descriptions for debugging

### 3. Response Format Compatibility
- ✅ **Schema Generation**: Automatic JSON schema creation from Rust structs
- ✅ **Deserialization**: Proper parsing of provider-specific response formats
- ✅ **Validation**: Ensures responses meet conventional commit standards

## Continuous Integration

Add these commands to your CI/CD pipeline:

```yaml
# .github/workflows/test.yml
- name: Run Unit Tests
  run: cargo test --lib

- name: Run Integration Tests
  run: cargo test --test integration_tests --ignored
  env:
    OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
    GOOGLE_AI_API_KEY: ${{ secrets.GOOGLE_AI_API_KEY }}
    ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
```

## Provider-Specific Notes

### OpenAI
- Uses `async-openai` crate for type safety
- Implements function calling with `tools` parameter
- Supports gpt-4, gpt-3.5-turbo models
- Rate limit: 3,500 requests/minute (tier 1)

### Gemini
- Uses direct HTTP requests with reqwest
- Implements function declarations with `tool_config`
- Supports gemini-1.5-flash, gemini-1.5-pro models
- Rate limit: 60 requests/minute (free tier)

### Anthropic
- Uses direct HTTP requests with reqwest
- Implements tool calling with `input_schema`
- Supports claude-3-5-sonnet-20241022, claude-3-haiku models
- Rate limit: 5 requests/minute (free tier), 1000/minute (paid)

## Troubleshooting

### Common Issues

1. **"Expected tool calls from OpenAI"**
   - ✅ **Fixed**: Updated to use proper function calling

2. **"Failed to parse Gemini response"**
   - ✅ **Fixed**: Added function call parsing with text fallback

3. **"Invalid response structure from Anthropic"**
   - ✅ **Fixed**: Updated to latest tool calling format

4. **Schema generation errors**
   - Ensure `schemars` dependency is available
   - Check struct derives include `JsonSchema`

### API Key Issues

```bash
# Verify API key format
echo $OPENAI_API_KEY | grep "^sk-"      # Should start with sk-
echo $GOOGLE_AI_API_KEY | wc -c          # Should be ~40 characters  
echo $ANTHROPIC_API_KEY | grep "^sk-ant-" # Should start with sk-ant-
```

## Next Steps

After running tests successfully:

1. **Deploy with Confidence**: All providers are verified working
2. **Monitor in Production**: Set up logging for API responses
3. **Update Tests Regularly**: Keep up with provider API changes
4. **Add New Providers**: Use existing test structure as template

For questions or issues, check the [main README](README.md) or open an issue on GitHub. 