import tkinter as tk

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y


class GameOfLifeLogic:
    def __init__(self, cells=set()):
        self.cells = cells
        self.next_set = None

    def tick(self):
        self.cells = self.next()
        self.next_set = None

    def next(self):
        if self.next_set:
            return self.next_set
        else:
            ret_set = set()
            covered = set()

            for old_cell in self.cells:
                if self.survives(old_cell):
                    ret_set.add(old_cell)

                for x, y in [(x, y)
                 for y in range(old_cell.y - 1, old_cell.y + 2)
                 for x in range(old_cell.x - 1, old_cell.x + 2)]:
                    covered.add(old_cell)
                    p = Point(x, y)
                    if p not in covered and p not in self.cells and self.living_neighbours(p) == 3:
                        ret_set.add(p)
        self.next_set = ret_set
        return ret_set


    def survives(self, point):
        if self.living_neighbours(point) in (0, 1): return False
        if self.living_neighbours(point) in (2, 3): return True
        else:                                       return False

    def living_neighbours(self, point):
        neighbours = 0

        for x, y in [(x, y)
         for y in range(point.y - 1, point.y + 2)
         for x in range(point.x - 1, point.x + 2)]:
            p = Point(x, y)
            if p in self.cells and p != point:
                neighbours += 1
        return neighbours


class GameOfLife(tk.Tk):
    def __init__(self, goll):
        self.goll = goll
        super()

    def tick(self):
        self.goll.tick()
    def draw(self):

goll = GameOfLifeLogic()
goll.cells.add(Point(34, 23))
goll.cells.add(Point(34, 24))
goll.cells.add(Point(34, 25))
goll.tick()
print(len(goll.cells))
