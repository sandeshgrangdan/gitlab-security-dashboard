export function initCharts(elementId,xValues,critical,high,medium,low,info,unknown) {
    require.config({
        paths: {
            'Chart': 'https://cdnjs.cloudflare.com/ajax/libs/Chart.js/2.9.4/Chart'
        }
    });

    require(['Chart'], function (Chart) {
        if (typeof Chart === 'undefined') {
            console.error('Chart.js is not loaded!');
            return;
        }

        console.log(elementId);
        console.log(xValues);
        console.log(critical);
        console.log(high);
        console.log(medium);
        console.log(low);

        new Chart(elementId, {
            type: "line",
            data: {
                labels: xValues,
                datasets: [{
                    data: critical,
                    borderColor: 'rgb(204, 85, 66)',
                    label: 'Critical',
                    fill: false,
                    tension: 0.3
                }, {
                    data: high,
                    borderColor: "orange",
                    label: 'High',
                    fill: false,
                    tension: 0.3
                }, {
                    data: medium,
                    label: 'Medium',
                    borderColor: "green",
                    fill: false,
                    tension: 0.3
                },
                {
                    label: 'Low',
                    data: low,
                    borderColor: "pink",
                    fill: false,
                    tension: 0.3
                },
                {
                    label: 'Info',
                    data: info,
                    borderColor: "rgb(75, 192, 192)",
                    fill: false,
                    tension: 0.3
                },
                {
                    label: 'Unknown',
                    data: unknown,
                    borderColor: "white",
                    fill: false,
                    tension: 0.3
                }
            ],
                
            },
            options: {
                legend: { display: true },
                scales: {
                    xAxes: [{
                      ticks: {
                        maxRotation: 0,
                        minRotation: 0,
                        callback: function(value, index, values) {
                        let date = value.split("-");
                          return index % 4 === 0 ? value : '';
                        }
                      }
                    }]
                }
            }
        });

        console.log("Chart initialized with elementId:", elementId);
    });
}