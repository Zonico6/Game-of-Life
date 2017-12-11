import java.awt.*
import java.awt.event.KeyEvent
import java.awt.event.KeyListener
import java.awt.event.MouseEvent
import java.awt.event.MouseListener
import java.util.*
import javax.swing.JFrame

typealias PointSet = HashSet<Point>

data class Point(val x: Int, val y: Int)

open class GameOfLifeLogic(var points: PointSet = PointSet()) {
    private var next: PointSet? = null

    fun tick() {
        this.points = next()
        next = null
    }

    fun add(point: Point) = points.add(point)
    fun remove(point: Point) = points.remove(point)

    fun cast() = points

    fun next(): PointSet {
        return if (next == null) {
                val retSet = PointSet()
                val covered = PointSet()

                points.filterTo(retSet) { it.survives(this.cast()) }

                // TODO: Optimize: In performance as well as readability
                for (point in points) {
                    for (x in point.x-1 .. point.x+1) {
                        for (y in point.y-1 .. point.y+1) {
                        	covered.add(point)

                            val p = Point(x, y)
                            if (!covered.contains(p) &&
                            	!points.contains(Point(x, y)) &&
                            	p.livingNeighbours(this.cast()) == 3) {
                                if (p.livingNeighbours(this.cast()) == 3)
                                    retSet.add(p)
                            }
                        }
                    }
                }

            retSet
        } else {
            next!!
        }
    }
    private fun Point.survives(points: PointSet) = when (livingNeighbours(points)) {
        0, 1 -> false
        2, 3 -> true
        else -> false
    }
    private fun Point.livingNeighbours(points: PointSet): Int {
        var neighbours = 0
        for (x in this.x-1 .. this.x + 1) {
            for (y in this.y-1 .. this.y+1) {
                val p = Point(x, y)
                if (points.contains(p) && this != p) {
                    neighbours += 1
                }
            }
        }
        return neighbours
    }
}

var p1: Point? = null

class GameOfLife(val logic: GameOfLifeLogic, width: Int, height: Int, private val rows: Int, private val columns: Int) : Canvas() {
    init {
        setSize(width, height)
    }

    val cellWidth: Int
        get() = width / rows
    val cellHeight: Int
        get() = height / columns

    fun update() {
        tick()
        repaint()
    }

    fun tick() {
        logic.tick()
    }

    override fun paint(g: Graphics?) {
        super.paint(g)

        // Draw points
        for ((x, y) in logic.points) {
            // FIXME: Those lines shouldn't produce different behavior...
            g!!.fillRect(x * cellWidth, y * cellHeight, cellWidth, cellHeight)
            // g!!.fillRect(x * width / rows, y * height / columns, width / rows, height / columns)
        }

        g!!.color = Color.red
        val rectSize = 8
        g.drawRect(p1?.x ?: -rectSize, p1?.y ?: -rectSize, rectSize, rectSize)

        g.color = Color.black

        /*
        // Draw the grid
        val htOfRow = height / rows
        for (i in 0..rows)
            g!!.drawLine(0, i * htOfRow, width, i * htOfRow)

        val wdOfRow = width / columns
        for (i in 0..columns)
            g!!.drawLine(i * wdOfRow, 0, i * wdOfRow, height)*/
    }
}

fun snap(pValue: Float, snap: Float): Float {
    var value = pValue
    val gap: Float
    var negFlag = false

    if (value < 0) {
        value = -value
        negFlag = true
    }
    gap = value % snap

    if (gap < snap / 2)
        value -= gap
    else
        value += snap - gap

    if (negFlag)
        return -value
    return value
}

fun main(args: Array<String>) {
    val windowWidth = 600
    val windowHeight = 600
    val rows = 100
    val columns = 100

    fun randomPoint(random: Random) = Point(random.nextInt(rows), random.nextInt(columns))

    val points_ = PointSet()
    for (i in 0..1000) {
        points_.add(randomPoint(Random()))
    }

    val points = GameOfLifeLogic(points_)
    val gol = GameOfLife(points, windowWidth, windowHeight, rows, columns)
    val frame = JFrame()

    fun MouseEvent.cell() = Point(
            snap(x.toFloat(), gol.cellWidth.toFloat()).toInt() / gol.cellWidth,
            snap(y.toFloat(), gol.cellHeight.toFloat()).toInt() / gol.cellHeight
    )

    frame.setSize(windowWidth, windowHeight)
    frame.addKeyListener(object : KeyListener {
        override fun keyTyped(e: KeyEvent?) {
            if (e!!.keyChar == 's')
                for (i in 1..99)
                    gol.update()
            gol.update()
        }
        override fun keyPressed(e: KeyEvent?) {}
        override fun keyReleased(e: KeyEvent?) {}
    })
    gol.addMouseListener(object : MouseListener {
        override fun mouseClicked(e: MouseEvent) {
            println("Mouse Clicked")
            if (e.button == MouseEvent.BUTTON1) {
                p1 = Point(e.x, e.y)

                gol.logic.add(e.cell())
                gol.repaint()
            }
        }
        override fun mouseEntered(e: MouseEvent?) {println("Mouse Entered")}
        override fun mouseExited(e: MouseEvent?) {println("Mouse Exited")}
        override fun mousePressed(e: MouseEvent?) {println("Mouse Pressed")}
        override fun mouseReleased(e: MouseEvent?) {println("Mouse Released")}
    })

    frame.add(gol)
    frame.title = "Conway's Game of Life"
    frame.defaultCloseOperation = JFrame.EXIT_ON_CLOSE
    gol.isFocusable = false
    frame.isVisible = true
}