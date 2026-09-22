export type NodeSide = 'left' | 'right' | 'top' | 'bottom';

export interface PositionDTO {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface CanvasNodeDTO {
  id: string;
  itemId?: string;
  localCellPath?: string;
  x: number;
  y: number;
  width: number;
  height: number;
  title?: string;
  content?: string;
  colorPreset?: string;
}

export interface CanvasEdgeDTO {
  id: string;
  sourceNodeId: string;
  targetNodeId: string;
  fromSide: NodeSide;
  toSide: NodeSide;
  label?: string;
  directed: boolean;
}

export interface ViewportStateDTO {
  x: number;
  y: number;
  zoom: number;
}

export interface CanvasTopologyDTO {
  workspaceId: string;
  viewport: ViewportStateDTO;
  nodes: CanvasNodeDTO[];
  edges: CanvasEdgeDTO[];
  updatedAt: number;
}

export interface CellDTO {
  id: string;
  workspaceId: string;
  title: string;
  content: string;
  position: PositionDTO;
  tags: string[];
  createdAt: number;
  updatedAt: number;
}
