import React, { useContext, useMemo, useRef, useState, useCallback } from "react"
import { JumperlessStateContext } from './JumperlessState'
import { breadboardNode, JumperlessNode, validateNode, NetlistEntry, SupplySwitchPos, NamedNode } from "./jlctlapi"
import {
  useFloating,
  autoUpdate,
  arrow,
  offset,
  flip,
  shift,
  useHover,
  useFocus,
  useDismiss,
  useRole,
  useInteractions,
  FloatingPortal,
  FloatingArrow,
} from "@floating-ui/react"
import './ImageBoardView.scss'
import { InteractionContext } from "./interaction"
import { SelectionInfo } from "./SelectionInfo"
import { netlistNodeNets } from "./netlist"
import { CursorModeIndicator } from "./CursorModeIndicator"
import { computeLayout, computeNanoLayout } from "./ImageBoardView/connections"
import { useSetting } from "./Settings"
import { imagePath } from "./utils"

const SWITCH_OPTS: Array<SupplySwitchPos> = [
  '8V',
  '5V',
  '3.3V',
]

const SWITCH_LABELS: { [key in SupplySwitchPos]: string } = {
  '3.3V': '+3.3V',
  '5V': '+5V',
  '8V': '±8V',
}

const ROW_POSITIONS = [
  [407.19788, 1450.7103], [486.94028, 1450.7103], [566.68262, 1450.7103], [646.42456, 1450.7103], [726.16718, 1450.7103],
  [805.9093, 1450.7103], [885.65149, 1450.7103], [965.39386, 1450.7103], [1045.1365, 1450.7103], [1124.8782, 1450.7103],
  [1204.6207, 1450.7103], [1284.3629, 1450.7103], [1364.105, 1450.7103], [1443.8474, 1450.7103], [1523.5897, 1450.7103],
  [1603.332, 1450.7103], [1683.0741, 1450.7103], [1762.8163, 1450.7103], [1842.5588, 1450.7103], [1922.3011, 1450.7103],
  [2002.0427, 1450.7103], [2081.7854, 1450.7103], [2161.5281, 1450.7103], [2241.27, 1450.7103], [2321.0122, 1450.7103],
  [2400.7549, 1450.7103], [2480.4963, 1450.7103], [2560.2393, 1450.7103], [2639.9817, 1450.7103], [2719.7236, 1450.7103],
  [407.19788, 2153.8367], [486.94028, 2153.8367], [566.68256, 2153.8367], [646.4245, 2153.8367], [726.16711, 2153.8367],
  [805.90924, 2153.8367], [885.65143, 2153.8367], [965.39386, 2153.8367], [1045.1365, 2153.8367], [1124.8782, 2153.8367],
  [1204.6207, 2153.8367], [1284.3629, 2153.8367], [1364.105, 2153.8367], [1443.8473, 2153.8367], [1523.5896, 2153.8367],
  [1603.3319, 2153.8367], [1683.074, 2153.8367], [1762.8163, 2153.8367], [1842.5588, 2153.8367], [1922.3011, 2153.8367],
  [2002.0427, 2153.8367], [2081.7852, 2153.8367], [2161.5278, 2153.8367], [2241.27, 2153.8367], [2321.012, 2153.8367],
  [2400.7549, 2153.8367], [2480.4961, 2153.8367], [2560.239, 2153.8367], [2639.9817, 2153.8367], [2719.7236, 2153.8367]
]

const ROW_WIDTH = 79.74227100000007
const ROW_HEIGHT = 563.3254399999998

const ROW_CENTERS = ROW_POSITIONS.map(([x, y]) => [x + ROW_WIDTH / 2, y])

const ROW_INDEX_OFFSETS = [
  196,
  276,
  356,
  436,
  516,
]

function holePosTop(row: JumperlessNode & number, index: number): [number, number] {
  const [x, y] = ROW_CENTERS[row - 1]
  return [x, y + ROW_INDEX_OFFSETS[index]]
}

function holePosBottom(row: JumperlessNode & number, index: number): [number, number] {
  const [x, y] = ROW_CENTERS[row - 1]
  return [x, y + ROW_HEIGHT - ROW_INDEX_OFFSETS[index]]
}

function holePos(row: JumperlessNode & number, index: number): [number, number] {
  return row > 30 ? holePosBottom(row, index) : holePosTop(row, index)
}

const RAIL_POS = {
  tPos: [347.19583, 1278.9934],
  tNeg: [347.19583, 1364.8767],
  bPos: [349.44824, 2717.2117],
  bNeg: [349.44824, 2803.095]
}

function railNode(rail: string, supplySwitchPos: SupplySwitchPos): JumperlessNode | null {
  if (rail === 'tNeg' || rail === 'bNeg') {
    return 'GND'
  }
  if (supplySwitchPos === '3.3V') {
    return 'SUPPLY_3V3'
  }
  if (supplySwitchPos === '5V') {
    return 'SUPPLY_5V'
  }
  return null
}

const nanoCenterTop: [number, number] = [42, 175]
const nanoCenterBottom: [number, number] = [33, 75]

const NANO_NODES: { [key: string]: { width: number, height: number, x: number, y: number, center?: [number, number], node?: JumperlessNode } } = {
 
}

function nanoPos (node: NamedNode): [number, number] {
  for (const nanoNode of Object.values(NANO_NODES)) {
    if (nanoNode.node === node) {
      const center = nanoNode.center || [0, 0]
      return [nanoNode.x + center[0], nanoNode.y + center[1]]
    }
  }
  throw new Error(`BUG: nano node position not found for ${node}`)
}

const SPECIAL_FUNCTIONS: { [key: string]: { width: number, height: number, x: number, y: number, rotate?: number } } = {
  
}

function makePath(points: Array<[number, number]>): string {
  const [x, y] = points[0]
  return `M${x} ${y}` + points.slice(1).map(([x, y]) => `L${x} ${y}`).join('')
}

const ImageBoardView: React.FC = () => {
  const { netlist, supplySwitchPos, setSupplySwitchPos, busy } = useContext(JumperlessStateContext)
  const { mode, handleNodeClick, selectedNode, handleDismiss, cursorHint, setHighlightedNode, highlightedNet, setHighlightedNet } = useContext(InteractionContext)!
  const nodeNets = useMemo(() => netlistNodeNets(netlist), [netlist])
  const [australia] = useSetting('australia', false)

  const nodeColor = useCallback((node: JumperlessNode): string | null => {
    const net = nodeNets.get(node)
    return net ? net.color : null
  }, [nodeNets])

  const switchDiff = useMemo(() => {
    return (SWITCH_OPTS.indexOf(supplySwitchPos) - 1) * 55
  }, [supplySwitchPos])

  function cycleSwitchPos() {
    const i = (SWITCH_OPTS.indexOf(supplySwitchPos) + 1) % SWITCH_OPTS.length
    setSupplySwitchPos(SWITCH_OPTS[i])
  }

  const { arrow, refs, getReferenceProps, getFloatingProps, isOpen, floatingStyles } = useRailswitchTooltip()

  function elementNode(element: HTMLElement): JumperlessNode | null {
    const node = element.dataset.node

    if (node) {
      return validateNode(node)
    }
    return null
  }

  const handleClick = (e: React.MouseEvent<SVGSVGElement>) => {
    handleNodeClick(elementNode(e.target as HTMLElement))
  }

  const handleHover = (e: React.MouseEvent<SVGSVGElement>) => {
    const node = elementNode(e.target as HTMLElement)
    setHighlightedNode(node)
    const net = node && nodeNets.get(node)
    setHighlightedNet(net ? net.index : null)
  }

  const handleContextMenu = (e: React.MouseEvent) => {
    if (handleDismiss()) {
      e.preventDefault()
    }
  }

  const rows = ROW_POSITIONS.map(([x, y], i) => {
    const row = i + 1
    const fill = nodeColor(breadboardNode(row))
    const style: React.CSSProperties = {}
    if (fill) {
      style.fill = fill
    }

    return (
      <rect
        key={row}
        x={x}
        y={y}
        id={`row${row}`}
        className={`row ${selectedNode === row ? 'selected' : ''}`}
        width={ROW_WIDTH}
        height={ROW_HEIGHT}
        style={style}
        data-node={row}
      />
    )
  })

  const rails = useMemo(() => {
    return Object.entries(RAIL_POS).map(([id, [x, y]]) => {
      const node = railNode(id, supplySwitchPos)
      const color = node && nodeColor(node)
      const style: React.CSSProperties = {}
      if (color) {
        style.fill = color
      } else if (id === 'tPos'){
        style.fill = "#faa000"
      } else if (id === 'bPos') {
        style.fill = "#8020fa"
      }
      return (
        <rect className='rail' key={id} id={id} width={2514.6069} height={85.783966} x={x} y={y} ry={0.31750244} style={style} data-node={node} />
      )
    })
  }, [supplySwitchPos, nodeColor])

  const specialMarkers = useMemo(() => {
    const markers: Array<React.ReactNode> = []
    netlist.forEach(net => {
      if (net.special) {
        net.nodes.slice(1).forEach(node => {
          markers.push(markerFor(net, node, australia))
        })
      }
    })
    return markers
  }, [netlist, australia])

  const connections = useMemo(() => {
    return computeLayout(netlist).map(({ a, b, netIndex }) => {
      let d: string

      if (a.node <= 30 && b.node <= 30) { // both in top half
        const aPos = holePosTop(a.node, a.index)
        const bPos = holePosTop(b.node, b.index)
        d = makePath(
          Math.abs(a.node - b.node) === 1
          ? [aPos, bPos] // direct neighbors get a straight connection
          : [ // all others are connected with midpoints
              aPos,
              [
                aPos[0] + (ROW_WIDTH / 2),
                aPos[1] + (ROW_WIDTH / 2),
              ],
              [
                bPos[0] - (ROW_WIDTH / 2),
                bPos[1] + (ROW_WIDTH / 2),
              ],
              bPos,
          ]
        )
      } else if (a.node > 30 && b.node > 30) { // both in bottom half
        const aPos = holePosBottom(a.node, a.index)
        const bPos = holePosBottom(b.node, b.index)
        d = makePath(
          Math.abs(a.node - b.node) === 1
          ? [aPos, bPos] // direct neighbors get a straight connection
          : [ // all others are connected with midpoints
              aPos,
              [
                aPos[0] + (ROW_WIDTH / 2),
                aPos[1] - (ROW_WIDTH / 2),
              ],
              [
                bPos[0] - (ROW_WIDTH / 2),
                bPos[1] - (ROW_WIDTH / 2),
              ],
              bPos,
          ]
        )
      } else {
        const aPos = holePosTop(a.node, a.index)
        const bPos = holePosBottom(b.node, b.index)
        d = makePath([
          aPos,
          [aPos[0], aPos[1] + (ROW_WIDTH / 2)],
          [bPos[0], bPos[1] - (ROW_WIDTH / 2)],
          bPos,
        ])
      }

      const style = {
        stroke: nodeColor(a.node)!,
        strokeWidth: netIndex === highlightedNet ? 16 : 8,
        fill: 'none',
      }
      const id = `${a.node}-${b.node}`
      return <path className='connection' key={id} id={id} style={style} d={d} />
    })
  }, [netlist, highlightedNet, nodeColor])

  const nanoPins = useMemo(() => {
    return Object.entries(NANO_NODES).map(([nanoNode, { x, y, width, height, node }]) => {
      const color = node ? nodeColor(node) : null
      const style: React.CSSProperties = {}
      if (color) {
        style.fill = color
      }
      const id = `nano-${nanoNode}`
      return <rect key={nanoNode} className='nanoPin' id={id} data-node={node} x={x} y={y} width={width} height={height} ry={0.37795275} style={style} />
    })
  }, [nodeColor])

  const nanoConnections = useMemo(() => {
    return computeNanoLayout(netlist).map(({ netIndex, a, b }) => {
      const aPos = typeof a.node === 'number' ? holePos(a.node, 0) : nanoPos(a.node)
      const bPos = typeof b.node === 'number' ? holePos(b.node, 0) : nanoPos(b.node)
      const d = makePath([aPos, bPos])
      const style = {
        stroke: nodeColor(a.node)!,
        strokeWidth: netIndex === highlightedNet ? 16 : 8,
        fill: 'none',
      }
      const id = `${a.node}-${b.node}`
      return <path className='connection' key={id} id={id} style={style} d = {d} />
    })
  }, [netlist, highlightedNet, nodeColor])

  const specialFunctions = useMemo(() => {
    return Object.entries(SPECIAL_FUNCTIONS).map(([node, { x, y, width, height, rotate }]) => {
      const transformProps = typeof rotate === 'number' ? { transform: `rotate(${rotate})` } : {}
      return (
        <rect
          className='specialFunction'
          key={node}
          x={x}
          y={y}
          width={width}
          height={height}
          {...transformProps}
        />
      )
    })
  }, [])

  return (
    <div className="ImageBoardView" data-busy={busy}>
      <SelectionInfo />
      {cursorHint !== null && <CursorModeIndicator hint={cursorHint} />}
      <svg
        viewBox="0 1000 3200 3200"
        width="100%"
        height="auto"
        preserveAspectRatio="xMidYMid"
        onClick={handleClick}
        onMouseMove={handleHover}
        data-interaction-mode={mode}
        onContextMenu={handleContextMenu}
      >
        <g id="g1">
          <g id="nanoHeader">{nanoPins}</g>
          <g id="rails">{rails}</g>
          <g id="Rows">{rows}</g>
          <g id="specialFunctions">{specialFunctions}</g>
          <rect
            style={{
              opacity: 1,
              fill: "#ff275d",
              fillRule: "evenodd",
              stroke: "#000",
              strokeWidth: 0.105827,
              strokeLinecap: "round",
              strokeLinejoin: "round",
              fillOpacity: 1
            }}

          />

          <image
            width={3242.8799}
            height={3242.8799}
            preserveAspectRatio="none"
            style={{ display: "inline", pointerEvents: 'none' }}
            xlinkHref={imagePath('board-background.png')}
            id="image1"
            x={0}
            y={-0.00001924485}
          />
          <g className="connections">{connections}</g>
          <g className="nanoConnections">{nanoConnections}</g>
          <g className="specialMarkers">{specialMarkers}</g>
        </g>
      </svg>
      <FloatingPortal>
        {isOpen && (
          <div
            className="railSwitchTooltip"
            ref={refs.setFloating}
            style={floatingStyles}
            {...getFloatingProps()}
          >
            {arrow}
            <div className='value'>{SWITCH_LABELS[supplySwitchPos]}</div>
          </div>
        )}
      </FloatingPortal>
    </div>
  )
}

const GND: React.FC<{ color: string, x: number, y: number, flip?: boolean }> = ({ color, x, y, flip }) => {
  const style = {
    stroke: color,
    strokeWidth: 4.12754,
    fill: 'none',
  }
  return (
    <g
      transform={`translate(${x} ${y}) ${flip ? 'scale(1 -1)' : ''}`}
    >
      <path
        style={style}
        d={`m 7,78 c 0,-1.0732 -0.49529999999999996,-1.651 -1.4859200000000001,-1.651 h -9.24571 c -1.07316,0 -1.5684699999999998,0.578 -1.5684699999999998,1.651 0,1.0732 0.49529999999999996,1.651 1.5684699999999998,1.651 h 9.24571 c 0.9905999999999999,0 1.4859200000000001,-0.578 1.4859200000000001,-1.651 z m 6.02623,-9.5759 c 0,-1.0732 -0.49529999999999996,-1.651 -1.4859200000000001,-1.651 h -21.29816 c -1.07316,0 -1.5684699999999998,0.578 -1.5684699999999998,1.651 0,1.0731 0.49529999999999996,1.651 1.5684699999999998,1.651 h 21.29816 c 0.9905999999999999,0 1.4859200000000001,-0.578 1.4859200000000001,-1.651 z m 6.02622,-9.4934 c 0,-1.0732 -0.49529999999999996,-1.651 -1.4859200000000001,-1.651 h -33.43315 c -1.07317,0 -1.5684699999999998,0.578 -1.5684699999999998,1.651 0,1.0732 0.49529999999999996,1.651 1.5684699999999998,1.651 h 33.43315 c 0.9905999999999999,0 1.4859200000000001,-0.578 1.4859200000000001,-1.651 z m 6.02622,-9.4934 c 0,-1.1557 -0.49529999999999996,-1.7335 -1.4859200000000001,-1.7335 h -21.13305 v -29.3056 c 0,-0.9909999999999999 -0.5779,-1.4860000000000002 -1.65102,-1.4860000000000002 -1.07317,0 -1.65102,0.495 -1.65102,1.4860000000000002 v 29.3056 h -21.05051 c -1.07316,0 -1.5684699999999998,0.578 -1.5684699999999998,1.7335 0,1.0732 0.49529999999999996,1.6511 1.5684699999999998,1.6511 h 45.485600000000005 c 0.9905999999999999,0 1.4859200000000001,-0.578 1.4859200000000001,-1.6511 z`}
        id="path34608" />
    </g>
  )
}

function markerFor(specialNet: NetlistEntry, node: JumperlessNode, flip: boolean): React.ReactNode {
  if (specialNet.name === 'GND' && typeof node === 'number') {
    const [x, y] = markerPos(node, flip)
    return <GND key={node} color={specialNet.color} x={x} y={y} flip={flip} />
  }
  return null
}

function markerPos(row: JumperlessNode & number, flip: boolean): [number, number] {
  if (row <= 30) {
    return holePosTop(row, flip ? 0 : 4)
  } else {
    return holePosBottom(row, flip ? 4 : 0)
  }
}

function useRailswitchTooltip() {
  const [isOpen, setIsOpen] = useState(false)
  const arrowRef = useRef(null)
  const { refs, floatingStyles, context } = useFloating({
    open: isOpen,
    onOpenChange: setIsOpen,
    placement: "left",
    whileElementsMounted: autoUpdate,
    middleware: [
      arrow({ element: arrowRef }),
      offset(5),
      flip({
        fallbackAxisSideDirection: "start"
      }),
      shift()
    ]
  });
  const hover = useHover(context, { move: false });
  const focus = useFocus(context);
  const dismiss = useDismiss(context);
  const role = useRole(context, { role: "tooltip" });
  const { getReferenceProps, getFloatingProps } = useInteractions([
    hover,
    focus,
    dismiss,
    role
  ]);

  return {
    arrow: <FloatingArrow ref={arrowRef} context={context} fill='#BF4040' />,
    isOpen,
    floatingStyles,
    refs,
    getReferenceProps,
    getFloatingProps,
  }
}

export default ImageBoardView
