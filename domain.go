package agora

const UniverseType = "Universe"

// Universe represents a set of Characters, Locations and Events
type Universe struct {
	Id   string `json:"uid,omitempty"`
	Name string `json:"Universe.name,omitempty"`
	User string `json:"Universe.user,omitempty"`
}
