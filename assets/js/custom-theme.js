(function (root, factory) {
    if (typeof define === 'function' && define.amd) {
        // AMD. Register as an anonymous module.
        define(['exports', 'echarts'], factory);
    } else if (typeof exports === 'object' && typeof exports.nodeName !== 'string') {
        // CommonJS
        factory(exports, require('echarts'));
    } else {
        // Browser globals
        factory({}, root.echarts);
    }
}(this, function (exports, echarts) {
    var log = function (msg) {
        if (typeof console !== 'undefined') {
            console && console.error && console.error(msg);
        }
    };
    if (!echarts) {
        log('ECharts is not Loaded');
        return;
    }
    const style = getComputedStyle(document.documentElement);
    
    // Resolve the CSS variables into real colors
    const colors = {
        bgPrimary: style.getPropertyValue('--bg-primary').trim(),
        bgSecondary: style.getPropertyValue('--bg-secondary').trim(),
        bgAccent: style.getPropertyValue('--bg-accent').trim(),

        fgPrimary: style.getPropertyValue('--fg-primary').trim(),
        fgSecondary: style.getPropertyValue('--fg-secondary').trim(),
        fgMuted: style.getPropertyValue('--fg-muted').trim(),

        accent: style.getPropertyValue('--accent-color').trim(),
        success: style.getPropertyValue('--success').trim(),
        error: style.getPropertyValue('--error').trim(),
        warning: style.getPropertyValue('--warning').trim()

    };

    echarts.registerTheme('Rejection Roulette', {
        "color": [
            colors.fgSecondary,
            colors.accent,
            colors.fgMuted,
            colors.success,
            colors.error,
            colors.warning
        ],
        "visualMap": {
            "type": "continuous",
            "show": false,
            "color": ["#f809ec", "#ff87f9", "#ffffff"]
        },
        "backgroundColor": "rgba(0,0,0,0)",
        "label": {
            "color": colors.fgPrimary
        },
        "legend": {
            "textStyle": {
                "color": colors.fgPrimary
            }
        },
        "bar": {
            "itemStyle": {
                "barBorderWidth": 0,
                "barBorderColor": "#ccc"
            }
        },
        "pie": {
            "itemStyle": {
                "borderWidth": 0,
                "borderColor": "#ccc"
            }
        },
        "sankey": {
            "label": {
                "show": true,
                "position": "right",
                "formatter": "{b}: {c}"
            },
            "itemStyle": {
                "borderWidth": 0,
                "borderColor": "#ccc"
            }
        },
        "calendar": {
            "yearLabel": {
                "show": true,
                "color": colors.fgPrimary,
            },
            "monthLabel": {
                "show": true,
                "color": colors.fgPrimary,
            },
            "dayLabel": {
                "show": true,
                "color": colors.fgPrimary,
            },
        },
        "categoryAxis": {
            "axisLine": {
                "show": true,
                "lineStyle": {
                    "color": "#cccccc"
                }
            },
            "axisTick": {
                "show": false,
                "lineStyle": {
                    "color": "#333"
                }
            },
            "axisLabel": {
                "show": true,
                "color": "#999999"
            },
            "splitLine": {
                "show": true,
                "lineStyle": {
                    "color": [
                        "#eeeeee"
                    ]
                }
            },
            "splitArea": {
                "show": false,
                "areaStyle": {
                    "color": [
                        "rgba(250,250,250,0.05)",
                        "rgba(200,200,200,0.02)"
                    ]
                }
            }
        },
        "valueAxis": {
            "axisLine": {
                "show": true,
                "lineStyle": {
                    "color": "#cccccc"
                }
            },
            "axisTick": {
                "show": false,
                "lineStyle": {
                    "color": "#333"
                }
            },
            "axisLabel": {
                "show": true,
                "color": "#999999"
            },
            "splitLine": {
                "show": true,
                "lineStyle": {
                    "color": [
                        "#eeeeee"
                    ]
                }
            },
            "splitArea": {
                "show": false,
                "areaStyle": {
                    "color": [
                        "rgba(250,250,250,0.05)",
                        "rgba(200,200,200,0.02)"
                    ]
                }
            }
        },
        "logAxis": {
            "axisLine": {
                "show": true,
                "lineStyle": {
                    "color": "#cccccc"
                }
            },
            "axisTick": {
                "show": false,
                "lineStyle": {
                    "color": "#333"
                }
            },
            "axisLabel": {
                "show": true,
                "color": "#999999"
            },
            "splitLine": {
                "show": true,
                "lineStyle": {
                    "color": [
                        "#eeeeee"
                    ]
                }
            },
            "splitArea": {
                "show": false,
                "areaStyle": {
                    "color": [
                        "rgba(250,250,250,0.05)",
                        "rgba(200,200,200,0.02)"
                    ]
                }
            }
        },
        "timeAxis": {
            "axisLine": {
                "show": true,
                "lineStyle": {
                    "color": "#cccccc"
                }
            },
            "axisTick": {
                "show": false,
                "lineStyle": {
                    "color": "#333"
                }
            },
            "axisLabel": {
                "show": true,
                "color": "#999999"
            },
            "splitLine": {
                "show": true,
                "lineStyle": {
                    "color": [
                        "#eeeeee"
                    ]
                }
            },
            "splitArea": {
                "show": false,
                "areaStyle": {
                    "color": [
                        "rgba(250,250,250,0.05)",
                        "rgba(200,200,200,0.02)"
                    ]
                }
            }
        },
        "toolbox": {
            "iconStyle": {
                "borderColor": "#999"
            },
            "emphasis": {
                "iconStyle": {
                    "borderColor": "#666"
                }
            }
        },
        "tooltip": {
            "axisPointer": {
                "lineStyle": {
                    "color": "#cccccc",
                    "width": 1
                },
                "crossStyle": {
                    "color": "#cccccc",
                    "width": 1
                }
            }
        },
        "timeline": {
            "lineStyle": {
                "color": "#8fd3e8",
                "width": 1
            },
            "itemStyle": {
                "color": "#8fd3e8",
                "borderWidth": 1
            },
            "controlStyle": {
                "color": "#8fd3e8",
                "borderColor": "#8fd3e8",
                "borderWidth": 0.5
            },
            "checkpointStyle": {
                "color": "#8fd3e8",
                "borderColor": "#8a7ca8"
            },
            "label": {
                "color": "#8fd3e8"
            },
            "emphasis": {
                "itemStyle": {
                    "color": "#8fd3e8"
                },
                "controlStyle": {
                    "color": "#8fd3e8",
                    "borderColor": "#8fd3e8",
                    "borderWidth": 0.5
                },
                "label": {
                    "color": "#8fd3e8"
                }
            }
        },
        "dataZoom": {
            "backgroundColor": "rgba(0,0,0,0)",
            "dataBackgroundColor": "rgba(255,255,255,0.3)",
            "fillerColor": "rgba(167,183,204,0.4)",
            "handleColor": "#a7b7cc",
            "handleSize": "100%",
            "textStyle": {
                "color": "#333"
            }
        },
        "markPoint": {
            "label": {
                "color": "#eee"
            },
            "emphasis": {
                "label": {
                    "color": "#eee"
                }
            }
        }
    });
}));