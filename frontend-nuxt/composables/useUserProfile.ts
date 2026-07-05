export type UserProfile = {
  userId: string;
  companyId: string;
  activityCount: number;
  recentActivity: Array<Record<string, unknown>>;
  actionCounts: Record<string, number>;
};

/** HTTP boundary — in the Nuxt app this is `$fetch` (ofetch). */
export type Fetcher = (url: string) => Promise<unknown>;

/**
 * Loads a user's profile rollup
 * (`GET /api/companies/:companyId/users/:userId/profile`).
 */
export async function fetchUserProfile(
  fetcher: Fetcher,
  companyId: string,
  userId: string,
): Promise<UserProfile> {
  const data = await fetcher(`/api/companies/${companyId}/users/${userId}/profile`);
  return data as UserProfile;
}
