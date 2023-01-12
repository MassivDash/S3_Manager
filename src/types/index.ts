export * from './files';
export * from './images';
export type GridCol = 1 | 2 | 3 | 4;

export interface TauriError {
    message: string;
    name: string;
}

export type VirtualListArray<Type> = Type extends Array<infer Item> ? Item[] : Type[];
export type VirtualListType<Type> = Type extends Array<infer Item> ? Item : Type;
