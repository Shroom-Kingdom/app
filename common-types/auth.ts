export interface NearRegister {
  username: string;
  walletId: string;
}

export interface Account {
  username: string;
  walletId: string;
}

export interface DiscordUser {
  id: string;
  username: string;
  discriminator: string;
  createdAt: string;
  isMember: boolean;
}
