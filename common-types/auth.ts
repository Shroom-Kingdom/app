export interface Account {
  uuid: string;
  username: string;
  walletId?: string;
  discord?: DiscordUser;
}

export interface DiscordUser {
  id: string;
  username: string;
  discriminator: string;
  createdAt: string;
  isMember: boolean;
}
