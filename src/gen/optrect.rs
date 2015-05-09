/*
within one chunk:
  rectangles = empty list
  maintain for each tile a boolean whether it's merged yet
  area = 0
  while area < total chunk area
    let rect
    a': for y in 0..max y
      for x in 0..max x
        if tiles[y][x] not merged
          let rect = tiles[y][x]
          break 'a
    
    now rect is coextensive with the first unmerged tile
    
    b': loop
      success = false
      if expand rect right
        success = true
      if expand rect down
        success = true
      if not success
        break 'b
    
    for y in rect min y..rect max y
      for x in rect min x..rect max x
        tiles[y][x] merged = true
    
    area = area + rect area
    
    rectangles push rect
*/