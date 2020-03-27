pub struct Table {
    name: String,
    symbol: String,
    charts: Charts,
}

pub struct Charts {
    charts: Vec<Chart>
}

pub struct Chart
{
    title: String,
    artist: String,
    md5: String,
}

impl Table {
    pub fn new(name: String, symbol: String, charts: Charts) -> Table {
        Table { name, symbol, charts }
    }

    pub fn string(&self) -> String {
        format!("{} [{}] {}", self.name, self.symbol, self.charts.string())
    }
}

impl Charts {
    pub fn new(charts: Vec<Chart>) -> Charts {
        Charts { charts }
    }

    pub fn string(&self) -> String {
        let vec: Vec<String> = self.charts.iter().map(|c| c.string()).collect();
        vec.join("\n")
    }
}

impl Chart {
    pub fn new(title: String, artist: String, md5: String) -> Chart {
        Chart { title, artist, md5 }
    }

    pub fn string(&self) -> String {
        format!("{}: {}", self.title, self.artist)
    }
}