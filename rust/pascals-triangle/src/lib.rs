pub struct PascalsTriangle {
    row_count: u32
}


impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{row_count}
    }

    fn next_row(&self, input_row: Option<Vec<u32>>) -> Vec<u32> {
        match input_row  {
            None => vec![1],
            Some(q) => {
                let b:Vec<u32> = [q.clone(), vec![0]].concat();
                let a:Vec<u32> = [vec![0], q].concat();
                a.iter().zip(b.iter()).map(|(x,y)|x+y).collect()
            }
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut pt:Vec<Vec<u32>> = vec![];

        while pt.len() < (self.row_count as usize) {
            let last_row = pt.last();
            let nr = self.next_row(last_row);

            pt.push(nr);
        }

        pt
    }
}
