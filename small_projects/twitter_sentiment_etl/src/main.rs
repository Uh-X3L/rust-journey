use vader_sentiment::SentimentIntensityAnalyzer;
use rusqlite::{params, Connection, Result};

fn analyze_sentiment(text: &str, analyzer: &SentimentIntensityAnalyzer) -> f64 {
    let scores = analyzer.polarity_scores(text);
    scores["compound"]
}

fn save_to_sqlite(conn: &Connection, tweet_id: &str, text: &str, sentiment_score: f64) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO sentiment (id, text, score) VALUES (?1, ?2, ?3)",
        params![tweet_id, text, sentiment_score],
    )?;
    Ok(())
}
let replacements = [
    // Emojis
    ("🚀", "moon"),
    ("📈", "uptrend"),
    ("📉", "downtrend"),
    ("💥", "crash"),
    ("🔥", "hot"),
    ("😢", "sad"),
    ("😭", "crying"),
    ("😡", "angry"),
    ("🤯", "mind blown"),
    ("💎", "diamond hands"),
    ("🙌", "cheering"),
    ("🤑", "greedy"),
    ("😎", "confident"),
    ("🫡", "respect"),
    ("💰", "money"),
    ("🧠", "smart"),
    ("🫠", "melting down"),
    ("📉", "loss"),
    ("🤡", "clown"),
    ("😱", "panic"),
    ("🫨", "shocked"),
    ("👀", "watching"),
    ("✅", "confirmed"),
    ("❌", "rejected"),
    
    // Crypto slang
    ("HODL", "hold"),
    ("FOMO", "fear of missing out"),
    ("FUD", "fear uncertainty doubt"),
    ("rekt", "lost money"),
    ("moon", "skyrocket"),
    ("bagholder", "holding a losing coin"),
    ("diamond hands", "strong holder"),
    ("paper hands", "weak holder"),
    ("bullish", "positive"),
    ("bearish", "negative"),
    ("pump", "increase rapidly"),
    ("dump", "sell-off"),
    ("LFG", "let's go"),
    ("WAGMI", "we are gonna make it"),
    ("NGMI", "not gonna make it"),
    ("degen", "high risk gambler"),
    ("alpha", "insider info"),
    ("rugpull", "scam"),
    ("to the moon", "increase a lot"),
    ("buy the dip", "buy when price falls"),
    ("flippening", "Ethereum overtakes Bitcoin"),
    ("ape in", "invest recklessly"),
    ("shitcoin", "worthless token"),
    ("moonboy", "overly optimistic person"),
];

fn clean_text(text: &str) -> String {
    let replacements = [/* insert above list here */];
    let mut cleaned = text.to_string();
    for (pattern, replacement) in replacements {
        cleaned = cleaned.replace(pattern, replacement);
    }
    cleaned
}


fn main() -> Result<()> {
    // Mock tweets (you can replace this later with real data)
    let tweets = vec![
        ("1", "Bitcoin is pumping! 🚀 #BTC"),
        ("2", "I'm losing so much money on crypto 😢"),
        ("3", "Holding steady. Not sure what will happen next."),
    ];

    // Setup analyzer
    let analyzer = SentimentIntensityAnalyzer::new();

    // Connect to SQLite DB
    let conn = Connection::open("tweets.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sentiment (
            id TEXT PRIMARY KEY,
            text TEXT,
            score REAL
        )",
        [],
    )?;

    // Run ETL
    for (id, text) in tweets {
        let cleaned_text = clean_text(text);
        let score = analyze_sentiment(&cleaned_text, &analyzer);
        println!("Original Tweet: {}\nCleaned: {}\nScore: {}\n", text, cleaned_text, score);
        save_to_sqlite(&conn, id, text, score)?;
    }

    println!("✅ All tweets processed and stored.");

    Ok(())
}
