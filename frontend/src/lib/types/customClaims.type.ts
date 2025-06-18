export type Permission = string;

export type Group = {
    id: string;
    name: string;
    permissions: Permission[];
};

export type CustomClaims = {
    groups: Group[];
};