export interface Project {
  allowModDistribution: boolean
  authors: Author[]
  categories: Category[]
  classId: number
  dateCreated: string
  dateModified: string
  dateReleased: string
  downloadCount: number
  gameId: number
  gamePopularityRank: number
  id: number
  isAvailable: boolean
  isFeatured: boolean
  latestFiles: LatestFile[]
  latestFilesIndexes: LatestFilesIndex[]
  links: Links
  logo: Logo
  mainFileId: number
  name: string
  primaryCategoryId: number
  screenshots: Screenshot[]
  slug: string
  status: number
  summary: string
  thumbsUpCount: number
}

export interface Screenshot {
  description?: string
  id: number
  modId: number
  thumbnailUrl: string
  title: string
  url: string
}

export interface Logo {
  description?: any
  id: number
  modId: number
  thumbnailUrl: string
  title: string
  url: string
}

export interface Links {
  issuesUrl: string
  sourceUrl?: any
  websiteUrl: string
  wikiUrl: string
}

export interface LatestFilesIndex {
  fileId: number
  filename: string
  gameVersion: string
  gameVersionTypeId: number
  modLoader?: any
  releaseType: number
}

export interface LatestFile {
  alternateFileId: number
  dependencies: any[]
  displayName: string
  downloadCount: number
  downloadUrl?: any
  exposeAsAlternative: boolean
  fileDate: string
  fileFingerprint: number
  fileLength: number
  fileName: string
  fileStatus: number
  gameId: number
  gameVersions: string[]
  hashes: Hash[]
  id: number
  isAvailable: boolean
  isServerPack: boolean
  modId: number
  modules: Module[]
  parentProjectFileId?: any
  releaseType: number
  serverPackFileId?: number
  sortableGameVersions: SortableGameVersion[]
}

export interface SortableGameVersion {
  gameVersion?: string | string
  gameVersionName: string
  gameVersionPadded: string
  gameVersionReleaseDate: string
  gameVersionTypeId: number
}

export interface Module {
  fingerprint: number
  name: string
}

export interface Hash {
  algo: number
  value: string
}

export interface Category {
  classId: number
  dateModified: string
  gameId: number
  iconUrl: string
  id: number
  isClass: boolean
  name: string
  parentCategoryId: number
  slug: string
  url: string
}

export interface Author {
  id: number
  name: string
  url: string
}

export default Project