//! Tests for tokenization functionality based on SPEC.md natural language requirements

#[cfg(test)]
mod tests {
    use super::super::*;

    fn create_tokenizer() -> Tokenizer {
        Tokenizer::new().expect("Failed to create tokenizer")
    }

    #[test]
    fn test_basic_word_tokenization() {
        // Test: Basic Word Tokenization
        // Given: "Hello world!"
        // When: Tokenizing the text
        // Then: Should produce 3 tokens with correct types and intervals
        
        let tokenizer = create_tokenizer();
        let text = "Hello world!";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        assert_eq!(tokenized.tokens.len(), 3);

        // Token[0]: type=WORD, text="Hello", char_interval=(0,5)
        let token0 = &tokenized.tokens[0];
        assert_eq!(token0.index, 0);
        assert_eq!(token0.token_type, TokenType::Word);
        assert_eq!(token0.char_interval.start_pos, 0);
        assert_eq!(token0.char_interval.end_pos, 5);
        assert_eq!(&text[token0.char_interval.start_pos..token0.char_interval.end_pos], "Hello");

        // Token[1]: type=WORD, text="world", char_interval=(6,11)
        let token1 = &tokenized.tokens[1];
        assert_eq!(token1.index, 1);
        assert_eq!(token1.token_type, TokenType::Word);
        assert_eq!(token1.char_interval.start_pos, 6);
        assert_eq!(token1.char_interval.end_pos, 11);
        assert_eq!(&text[token1.char_interval.start_pos..token1.char_interval.end_pos], "world");

        // Token[2]: type=PUNCTUATION, text="!", char_interval=(11,12)
        let token2 = &tokenized.tokens[2];
        assert_eq!(token2.index, 2);
        assert_eq!(token2.token_type, TokenType::Punctuation);
        assert_eq!(token2.char_interval.start_pos, 11);
        assert_eq!(token2.char_interval.end_pos, 12);
        assert_eq!(&text[token2.char_interval.start_pos..token2.char_interval.end_pos], "!");
    }

    #[test]
    fn test_number_recognition() {
        // Test: Number Recognition
        // Given: "Price is $29.99 per item"
        // When: Tokenizing the text
        // Then: Should correctly identify numbers vs punctuation
        
        let tokenizer = create_tokenizer();
        let text = "Price is $29.99 per item";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Find and verify specific tokens
        let tokens: Vec<_> = tokenized.tokens.iter().map(|t| {
            let token_text = &text[t.char_interval.start_pos..t.char_interval.end_pos];
            (token_text, t.token_type)
        }).collect();

        // Check that "Price" is WORD
        assert!(tokens.iter().any(|(text, typ)| *text == "Price" && *typ == TokenType::Word));
        
        // Check that "$" is PUNCTUATION
        assert!(tokens.iter().any(|(text, typ)| *text == "$" && *typ == TokenType::Punctuation));
        
        // Check that "29" is NUMBER
        assert!(tokens.iter().any(|(text, typ)| *text == "29" && *typ == TokenType::Number));
        
        // Check that "." is PUNCTUATION
        assert!(tokens.iter().any(|(text, typ)| *text == "." && *typ == TokenType::Punctuation));
        
        // Check that "99" is NUMBER
        assert!(tokens.iter().any(|(text, typ)| *text == "99" && *typ == TokenType::Number));
    }

    #[test]
    fn test_acronym_detection() {
        // Test: Acronym Detection
        // Given: "The HTML/CSS/JS stack"
        // When: Tokenizing the text
        // Then: Should identify "HTML/CSS/JS" as a single ACRONYM token
        
        let tokenizer = create_tokenizer();
        let text = "The HTML/CSS/JS stack";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Find the HTML/CSS/JS token
        let acronym_token = tokenized.tokens.iter().find(|t| {
            let token_text = &text[t.char_interval.start_pos..t.char_interval.end_pos];
            token_text == "HTML/CSS/JS"
        });

        assert!(acronym_token.is_some(), "HTML/CSS/JS should be found as a token");
        let acronym_token = acronym_token.unwrap();
        assert_eq!(acronym_token.token_type, TokenType::Acronym);
    }

    #[test]
    fn test_newline_boundary_tracking() {
        // Test: Newline Boundary Tracking
        // Given: "First line\nSecond line"
        // When: Tokenizing the text
        // Then: The token "Second" should have first_token_after_newline=true
        
        let tokenizer = create_tokenizer();
        let text = "First line\nSecond line";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Find the "Second" token
        let second_token = tokenized.tokens.iter().find(|t| {
            let token_text = &text[t.char_interval.start_pos..t.char_interval.end_pos];
            token_text == "Second"
        });

        assert!(second_token.is_some(), "Second token should be found");
        let second_token = second_token.unwrap();
        assert!(second_token.first_token_after_newline, "Second token should be marked as first after newline");

        // Verify that "First" does not have first_token_after_newline set
        let first_token = tokenized.tokens.iter().find(|t| {
            let token_text = &text[t.char_interval.start_pos..t.char_interval.end_pos];
            token_text == "First"
        });
        
        assert!(first_token.is_some(), "First token should be found");
        let first_token = first_token.unwrap();
        assert!(!first_token.first_token_after_newline, "First token should not be marked as first after newline");
    }

    #[test]
    fn test_text_reconstruction() {
        // Test: Text Reconstruction
        // Given: "Hello, world! How are you?"
        // When: Tokenizing and then reconstructing text from token interval (0,4)
        // Then: Should return exactly "Hello, world!"
        
        let tokenizer = create_tokenizer();
        let text = "Hello, world! How are you?";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Token interval (0,4) should include: "Hello", ",", "world", "!"
        let token_interval = TokenInterval::new(0, 4).expect("Token interval creation failed");
        let reconstructed = tokenizer.tokens_text(&tokenized, &token_interval)
            .expect("Text reconstruction failed");

        assert_eq!(reconstructed, "Hello, world!");
    }

    #[test]
    fn test_token_interval_validation() {
        // Test: Token Interval Validation
        // Given: TokenizedText with tokens
        // When: Creating token interval with start_index=5, end_index=3
        // Then: Should raise error
        
        let result = TokenInterval::new(5, 3);
        assert!(result.is_err(), "Invalid token interval should return error");
        
        if let Err(e) = result {
            assert!(e.to_string().contains("Start index 5 must be < end index 3"));
        }
    }

    #[test]
    fn test_character_interval_mapping() {
        // Test: Character Interval Mapping
        // Given: Tokenized text "Test string"
        // When: Getting character interval for token interval (0,1)
        // Then: Should return correct character positions that map back to "Test"
        
        let tokenizer = create_tokenizer();
        let text = "Test string";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        let token_interval = TokenInterval::new(0, 1).expect("Token interval creation failed");
        let reconstructed = tokenizer.tokens_text(&tokenized, &token_interval)
            .expect("Text reconstruction failed");

        assert_eq!(reconstructed, "Test");
    }

    #[test]
    fn test_abbreviation_recognition() {
        // Test: Abbreviation Recognition
        // Given: "Dr. Smith went to St. Mary's hospital."
        // When: Detecting sentence boundaries
        // Then: Should identify only one sentence (not split at "Dr." or "St.")
        
        let tokenizer = create_tokenizer();
        let text = "Dr. Smith went to St. Mary's hospital.";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        let sentence_range = tokenizer.find_sentence_range(text, &tokenized.tokens, 0)
            .expect("Sentence range detection failed");

        // Should include the entire text as one sentence
        assert_eq!(sentence_range.start_index, 0);
        assert_eq!(sentence_range.end_index, tokenized.tokens.len());

        let sentence_text = tokenizer.tokens_text(&tokenized, &sentence_range)
            .expect("Sentence text reconstruction failed");
        assert_eq!(sentence_text, text);
    }

    #[test]
    fn test_mixed_abbreviations_and_sentences() {
        // Test: Mixed Abbreviations and Sentences
        // Given: "Mr. Bond asked why. Prof. X answered."
        // When: Detecting sentence boundaries
        // Then: Should identify two sentences
        
        let tokenizer = create_tokenizer();
        let text = "Mr. Bond asked why. Prof. X answered.";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Find first sentence
        let first_sentence = tokenizer.find_sentence_range(text, &tokenized.tokens, 0)
            .expect("First sentence range detection failed");

        let first_sentence_text = tokenizer.tokens_text(&tokenized, &first_sentence)
            .expect("First sentence text reconstruction failed");
        assert_eq!(first_sentence_text, "Mr. Bond asked why.");

        // Find second sentence
        let second_sentence = tokenizer.find_sentence_range(text, &tokenized.tokens, first_sentence.end_index)
            .expect("Second sentence range detection failed");

        let second_sentence_text = tokenizer.tokens_text(&tokenized, &second_sentence)
            .expect("Second sentence text reconstruction failed");
        assert_eq!(second_sentence_text, "Prof. X answered.");
    }

    #[test]
    fn test_newline_with_capitalization() {
        // Test: Newline with Capitalization
        // Given: "First sentence\nSecond sentence starts here"
        // When: Detecting sentence boundaries
        // Then: Should split into two sentences at the newline
        
        let tokenizer = create_tokenizer();
        let text = "First sentence\nSecond sentence starts here";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Find first sentence
        let first_sentence = tokenizer.find_sentence_range(text, &tokenized.tokens, 0)
            .expect("First sentence range detection failed");

        // Should end before "Second"
        assert!(first_sentence.end_index < tokenized.tokens.len());
        
        let first_sentence_text = tokenizer.tokens_text(&tokenized, &first_sentence)
            .expect("First sentence text reconstruction failed");
        assert_eq!(first_sentence_text, "First sentence");
    }

    #[test]
    fn test_newline_without_capitalization() {
        // Test: Newline without Capitalization
        // Given: "First line\nsecond line continues"
        // When: Detecting sentence boundaries
        // Then: Should NOT split at the newline (lowercase 's')
        
        let tokenizer = create_tokenizer();
        let text = "First line\nsecond line continues";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        let sentence_range = tokenizer.find_sentence_range(text, &tokenized.tokens, 0)
            .expect("Sentence range detection failed");

        // Should include the entire text as one sentence since there's no sentence-ending punctuation
        // and the newline is followed by lowercase
        assert_eq!(sentence_range.start_index, 0);
        assert_eq!(sentence_range.end_index, tokenized.tokens.len());
    }

    #[test]
    fn test_empty_text_handling() {
        // Test: Empty Text Handling
        // Given: Empty string
        // When: Tokenizing
        // Then: Should return empty TokenizedText
        
        let tokenizer = create_tokenizer();
        let text = "";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization of empty string failed");

        assert_eq!(tokenized.tokens.len(), 0);
        assert!(tokenized.is_empty());
    }

    #[test]
    fn test_complex_punctuation() {
        // Test complex punctuation scenarios
        let tokenizer = create_tokenizer();
        let text = "What?! That's amazing... isn't it?";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Should properly tokenize complex punctuation
        let tokens: Vec<_> = tokenized.tokens.iter().map(|t| {
            &text[t.char_interval.start_pos..t.char_interval.end_pos]
        }).collect();

        assert!(tokens.contains(&"What"));
        assert!(tokens.contains(&"?!"));
        assert!(tokens.contains(&"..."));
        assert!(tokens.contains(&"'"));
    }

    #[test]
    fn test_multiple_spaces_and_tabs() {
        // Test handling of multiple spaces and tabs
        let tokenizer = create_tokenizer();
        let text = "Word1    \t   Word2";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Should tokenize as "Word", "1", "Word", "2" (4 tokens) due to alphanumeric separation
        assert_eq!(tokenized.tokens.len(), 4);
        
        let token_texts: Vec<_> = tokenized.tokens.iter().map(|t| {
            &text[t.char_interval.start_pos..t.char_interval.end_pos]
        }).collect();
        
        assert_eq!(token_texts, vec!["Word", "1", "Word", "2"]);
    }

    #[test]
    fn test_token_interval_bounds_checking() {
        // Test that token interval bounds are properly checked
        let tokenizer = create_tokenizer();
        let text = "Short text";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Try to create interval that goes beyond available tokens
        let bad_interval = TokenInterval::new(0, tokenized.tokens.len() + 1);
        assert!(bad_interval.is_ok()); // Creating the interval is ok

        // But using it should fail
        let result = tokenizer.tokens_text(&tokenized, &bad_interval.unwrap());
        assert!(result.is_err(), "Should fail when token interval exceeds available tokens");
    }

    // SentenceIterator tests based on SPEC.md requirements

    #[test]
    fn test_basic_sentence_iteration() {
        // Test: Basic Sentence Iteration
        // Given: "First sentence. Second sentence! Third sentence?"
        // When: Iterating through sentences
        // Then: Should yield three separate sentence intervals
        
        let tokenizer = create_tokenizer();
        let text = "First sentence. Second sentence! Third sentence?";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        let sentence_iter = SentenceIterator::new(&tokenized, &tokenizer, 0)
            .expect("Failed to create sentence iterator");

        let sentences: Result<Vec<_>, _> = sentence_iter.collect();
        let sentences = sentences.expect("Sentence iteration failed");

        assert_eq!(sentences.len(), 3, "Should find exactly 3 sentences");

        // Verify sentence texts
        let sentence_texts: Vec<_> = sentences.iter().map(|interval| {
            tokenizer.tokens_text(&tokenized, interval).expect("Failed to reconstruct sentence text")
        }).collect();

        assert_eq!(sentence_texts[0], "First sentence.");
        assert_eq!(sentence_texts[1], "Second sentence!");
        assert_eq!(sentence_texts[2], "Third sentence?");
    }

    #[test]
    fn test_starting_mid_document() {
        // Test: Starting Mid-Document
        // Given: Tokenized text with sentence iterator starting at token 5
        // When: Getting next sentence
        // Then: Should return sentence interval starting from token 5, not from the beginning of that sentence
        
        let tokenizer = create_tokenizer();
        let text = "First sentence. Second sentence! Third sentence?";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        // Find token 5 (should be somewhere in the middle)
        let start_token = 5; // This should be in the second sentence
        let mut sentence_iter = SentenceIterator::new(&tokenized, &tokenizer, start_token)
            .expect("Failed to create sentence iterator");

        let first_interval = sentence_iter.next()
            .expect("Should have a sentence")
            .expect("Sentence iteration should succeed");

        // The interval should start at token 5
        assert_eq!(first_interval.start_index, start_token);

        // The text should be a partial sentence starting from that token
        let partial_text = tokenizer.tokens_text(&tokenized, &first_interval)
            .expect("Failed to reconstruct text");
        
        // Should not start with "First" or "Second" since we're starting mid-document
        assert!(!partial_text.starts_with("First"));
        assert!(!partial_text.starts_with("Second"));
    }

    #[test]
    fn test_empty_text_sentence_iteration() {
        // Test: Empty Text Handling
        // Given: Empty string
        // When: Creating sentence iterator and calling next()
        // Then: Should not panic and should return None immediately
        
        let tokenizer = create_tokenizer();
        let text = "";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization of empty string failed");

        let mut sentence_iter = SentenceIterator::new(&tokenized, &tokenizer, 0)
            .expect("Failed to create sentence iterator for empty text");

        let result = sentence_iter.next();
        assert!(result.is_none(), "Empty text should immediately return None");
    }

    #[test]
    fn test_sentence_iterator_out_of_bounds() {
        // Test: Out of bounds token position
        // Given: Tokenized text with 5 tokens
        // When: Creating sentence iterator starting at token 10
        // Then: Should return an error
        
        let tokenizer = create_tokenizer();
        let text = "Short text.";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        let result = SentenceIterator::new(&tokenized, &tokenizer, tokenized.tokens.len() + 5);
        assert!(result.is_err(), "Should fail when starting position is beyond token count");
    }

    #[test]
    fn test_sentence_iterator_at_end() {
        // Test: Starting at the end of the document
        // Given: Tokenized text 
        // When: Creating sentence iterator starting at the last token position
        // Then: Should return None immediately
        
        let tokenizer = create_tokenizer();
        let text = "Test sentence.";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        let mut sentence_iter = SentenceIterator::new(&tokenized, &tokenizer, tokenized.tokens.len())
            .expect("Failed to create sentence iterator at end position");

        let result = sentence_iter.next();
        assert!(result.is_none(), "Starting at end position should return None");
    }

    #[test]
    fn test_sentence_iterator_progressive() {
        // Test: Progressive iteration through sentences
        // Given: Multiple sentences
        // When: Iterating one by one
        // Then: Each call to next() should return the next sentence
        
        let tokenizer = create_tokenizer();
        let text = "One. Two! Three?";
        let tokenized = tokenizer.tokenize(text).expect("Tokenization failed");

        let mut sentence_iter = SentenceIterator::new(&tokenized, &tokenizer, 0)
            .expect("Failed to create sentence iterator");

        // First sentence
        let sentence1 = sentence_iter.next()
            .expect("Should have first sentence")
            .expect("First sentence iteration should succeed");
        let text1 = tokenizer.tokens_text(&tokenized, &sentence1).expect("Failed to get text");
        assert_eq!(text1, "One.");

        // Second sentence
        let sentence2 = sentence_iter.next()
            .expect("Should have second sentence")
            .expect("Second sentence iteration should succeed");
        let text2 = tokenizer.tokens_text(&tokenized, &sentence2).expect("Failed to get text");
        assert_eq!(text2, "Two!");

        // Third sentence
        let sentence3 = sentence_iter.next()
            .expect("Should have third sentence")
            .expect("Third sentence iteration should succeed");
        let text3 = tokenizer.tokens_text(&tokenized, &sentence3).expect("Failed to get text");
        assert_eq!(text3, "Three?");

        // No more sentences
        let sentence4 = sentence_iter.next();
        assert!(sentence4.is_none(), "Should be no more sentences");
    }
}
