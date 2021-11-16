package universe

// Universe represents a set of Characters, Locations and Events
type Universe struct {
	Id   string `json:"uid,omitempty"`
	Name string `json:"name,omitempty"`
	User string `json:"user,omitempty"`
}
