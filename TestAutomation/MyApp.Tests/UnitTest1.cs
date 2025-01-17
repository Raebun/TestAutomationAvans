using NUnit.Framework;
using MyApp;

namespace MyApp.Tests
{
    [TestFixture] // Test class
    public class Tests
    {
        private DetermineShippingCosts _determineShippingCosts;

        [SetUp] // Gets run before each test
        public void SetUp()
        {
            _determineShippingCosts = new DetermineShippingCosts();
        }

        [Test]
        public void ShippingCosts_WhenCalculatingShippingCostsIsFalse_ReturnsZero()
        {
            // Act
            var result = _determineShippingCosts.ShippingCosts(false, "Ground", 1000);

            // Assert
            // Assert.That(result, Is.EqualTo(0)); // If false -> Result = 0
            Assert.AreEqual(0, result);

        }

        [Test]
        public void ShippingCosts_WhenTotalPriceExceeds1500_ReturnsZero1()
        {
            // Act
            var result1 = _determineShippingCosts.ShippingCosts(true, "Ground", 1501);
            var result2 = _determineShippingCosts.ShippingCosts(true, "Ground", 1600);

            // Assert
            Assert.AreEqual(0, result1);
            Assert.AreEqual(0, result2);
        }

        [Test]
        public void ShippingCosts_WhenTotalPriceEqualsOrBelow1500ForGround_Returns100()
        {
            // Act
            var result = _determineShippingCosts.ShippingCosts(true, "Ground", 1500);
            var result2 = _determineShippingCosts.ShippingCosts(true, "Ground", 1409);
            var result3 = _determineShippingCosts.ShippingCosts(true, "Ground", 0);
            var result4 = _determineShippingCosts.ShippingCosts(true, "Ground", -500);

            // Assert
            Assert.AreEqual(100, result);
            Assert.AreEqual(100, result2);
            Assert.AreEqual(100, result3);
            Assert.AreEqual(100, result4);
        }

        [Test]
        public void ShippingCosts_WhenTypeOfShippingcostsIsUnknown_ReturnsZero()
        {
            // Act
            var result = _determineShippingCosts.ShippingCosts(true, "Unknown", 1000);

            // Assert
            Assert.AreEqual(0, result);

        }

        [Test]
        public void ShippingCosts_WhenTypeOfShippingCostsIsGround_Returns100()
        {
            // Act
            var result = _determineShippingCosts.ShippingCosts(true, "Ground", 1000);

            // Assert
            Assert.AreEqual(100, result);
        }

        [Test]
        public void ShippingCosts_WhenTypeOfShippingCostsIsInStore_Returns50()
        {
            // Act
            var result = _determineShippingCosts.ShippingCosts(true, "InStore", 1000);

            // Assert
            Assert.AreEqual(50, result);
        }

        [Test]
        public void ShippingCosts_WhenTypeOfShippingCostsIsNextDayAir_Returns250()
        {
            // Act
            var result = _determineShippingCosts.ShippingCosts(true, "NextDayAir", 1000);

            // Assert
            Assert.AreEqual(250, result);
        }

        [Test]
        public void ShippingCosts_WhenTypeOfShippingCostsIsSecondDayAir_Returns125()
        {
            // Act
            var result = _determineShippingCosts.ShippingCosts(true, "SecondDayAir", 1000);

            // Assert
            Assert.AreEqual(125, result);
        }

    }
}