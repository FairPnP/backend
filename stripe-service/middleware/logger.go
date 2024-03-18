package middleware

import (
	"time"

	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

type ginHands struct {
	Path       string
	Latency    time.Duration
	Method     string
	StatusCode int
	ClientIP   string
	MsgStr     string
	RequestID  string
	UserID     string
}

func ErrorLogger() gin.HandlerFunc {
	return ErrorLoggerT(gin.ErrorTypeAny)
}

func ErrorLoggerT(typ gin.ErrorType) gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Next()

		if !c.Writer.Written() {
			json := c.Errors.ByType(typ).JSON()
			if json != nil {
				c.JSON(-1, json)
			}
		}
	}
}

func Logger() gin.HandlerFunc {
	return func(c *gin.Context) {
		t := time.Now()
		// before request
		path := c.Request.URL.Path
		raw := c.Request.URL.RawQuery
		c.Next()
		// after request
		// latency := time.Since(t)
		// clientIP := c.ClientIP()
		// method := c.Request.Method
		// statusCode := c.Writer.Status()
		if raw != "" {
			path = path + "?" + raw
		}
		msg := c.Errors.String()
		if msg == "" {
			msg = "Request"
		}
		requestID, exists := c.Get("requestID")
		if !exists {
			requestID = ""
		}
		userID, exists := c.Get("userID")
		if !exists {
			userID = ""
		}

		cData := &ginHands{
			Path:       path,
			Latency:    time.Since(t),
			Method:     c.Request.Method,
			StatusCode: c.Writer.Status(),
			ClientIP:   c.ClientIP(),
			MsgStr:     msg,
			RequestID:  requestID.(string),
			UserID:     userID.(string),
		}

		logSwitch(cData)
	}
}

func logSwitch(data *ginHands) {
	switch {
	case data.StatusCode >= 400 && data.StatusCode < 500:
		addLogData(log.Warn(), data)
	case data.StatusCode >= 500:
		addLogData(log.Error(), data)
	default:
		addLogData(log.Info(), data)
	}
}

func addLogData(logEvent *zerolog.Event, data *ginHands) {
	e := logEvent.Str("method", data.Method).Str("path", data.Path).Dur("latency", data.Latency).Int("status", data.StatusCode).Str("client_ip", data.ClientIP)
	if data.RequestID != "" {
		e = e.Str("request_id", data.RequestID)
	}
	if data.UserID != "" {
		e = e.Str("user_id", data.UserID)
	}
	e.Msg(data.MsgStr)
}
