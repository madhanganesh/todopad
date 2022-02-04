package model

import "time"

type ReportRequest struct {
	From                 time.Time `json:"from"`
	To                   time.Time `json:"to"`
	GroupBy              string    `json:"groupby"`
	Tags                 []string  `json:"tags"`
	TimeZoneOffsetInSecs int       `json:"timezoneoffset"`
}

type ReportResponse struct {
	ReportRequest
	EffortsByGroup map[string]float32 `json:"efforts"`
}
