export type NodeSide = 'Top' | 'Bottom' | 'Left' | 'Right' | 'top' | 'bottom' | 'left' | 'right';

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
  nodeType?: 'text' | 'asset' | 'note';
  assetHash?: string;
  assetExtension?: string;
  revision?: number;
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

export interface BoardTopologyDTO {
  workspaceId: string;
  viewport: ViewportStateDTO;
  nodes: CanvasNodeDTO[];
  edges: CanvasEdgeDTO[];
  revision: number;
  updatedAt: number;
}

export type CanvasTopologyDTO = BoardTopologyDTO;

export interface CellRecordDTO {
  cellId: string;
  path: string;
  revision: number;
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

