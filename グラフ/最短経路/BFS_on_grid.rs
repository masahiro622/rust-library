const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
//const DIRECTIONS: [(isize, isize); 8] = [ (1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (-1, 1), (1, -1)];

fn grid_bfs(grid: &[Vec<char>], start: (usize, usize)) -> Vec<Vec<isize>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut dist = vec![vec![-1isize; w]; h];
    let mut que = VecDeque::new();
    dist[start.0][start.1] = 0;
    que.push_back(start);
    while let Some((x, y)) = que.pop_front() {
        for &(dx, dy) in &DIRECTIONS {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[nx][ny] == '#' || dist[nx][ny] != -1 {
                continue;
            }
            dist[nx][ny] = dist[x][y] + 1;
            que.push_back((nx, ny));
        }
    }
    dist
}

fn grid_bfs_with_path(
    grid: &[Vec<char>],
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let h = grid.len();
    let w = grid[0].len();
    const directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut dist = vec![vec![-1isize; w]; h];
    let mut parent = vec![vec![None; w]; h]; // 親ノードを記録
    let mut que = VecDeque::new();

    dist[start.0][start.1] = 0;
    que.push_back(start);

    while let Some((x, y)) = que.pop_front() {
        if (x, y) == goal {
            break;
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[nx][ny] == '#' || dist[nx][ny] != -1 {
                continue;
            }
            dist[nx][ny] = dist[x][y] + 1;
            parent[nx][ny] = Some((x, y)); // 親を記録
            que.push_back((nx, ny));
        }
    }

    // 経路が見つからない場合
    if dist[goal.0][goal.1] == -1 {
        return None;
    }

    // 経路を復元
    let mut path = Vec::new();
    let mut current = goal;

    while let Some(p) = parent[current.0][current.1] {
        path.push(current);
        current = p;
    }
    path.push(start); // スタート地点を追加
    path.reverse(); // 逆順にして start -> goal の順にする

    Some(path)
}
